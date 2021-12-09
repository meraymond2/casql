use crate::binary_reader::{BinaryReader, ByteOrder};
use crate::cas_err::CasErr;
use crate::postgres::postgis::ewkb;
use crate::postgres::row_iter::RowIter;
use std::collections::HashMap;
use std::io::Write;

const LEFT_SQUARE: &[u8] = "[".as_bytes();
const LEFT_BRACE: &[u8] = "{".as_bytes();
const RIGHT_SQUARE: &[u8] = "]".as_bytes();
const RIGHT_BRACE: &[u8] = "}".as_bytes();
const NEW_LINE: &[u8] = "\n".as_bytes();
const DOUBLE_QUOTE: &[u8] = "\"".as_bytes();
const COMMA: &[u8] = ",".as_bytes();
const COLON: &[u8] = ":".as_bytes();
const NULL: &[u8] = "null".as_bytes();

#[derive(Debug)]
enum Parser {
    Array,
    Bool,
    BigNum,
    Bytes,
    Float32,
    Float64,
    Int16,
    Int32,
    Int64,
    String,
    EWKB,
    Tid,
    Unknown,
}

struct JsonField {
    name: String,
    parser: Parser,
}

pub fn write_rows<Out>(
    rows: RowIter,
    dynamic_types: &HashMap<i32, String>,
    out: &mut Out,
) -> Result<(), CasErr>
where
    Out: Write,
{
    out.write(LEFT_SQUARE)?;
    let fields: Vec<JsonField> = rows
        .fields
        .iter()
        .map(|field| JsonField {
            name: field.name.clone(),
            parser: find_parser(field.data_type_oid, dynamic_types),
        })
        .collect();
    let mut first = true;
    for row in rows {
        if first {
            first = false
        } else {
            out.write(COMMA)?;
        }
        write_row(row, &fields, out)?;
    }
    out.write(RIGHT_SQUARE)?;
    out.write(NEW_LINE)?;
    out.flush()?;
    Ok(())
}

fn write_row<Out>(row: Vec<u8>, fields: &Vec<JsonField>, out: &mut Out) -> Result<(), CasErr>
where
    Out: Write,
{
    let mut rdr = BinaryReader::from(&row, ByteOrder::BigEndian);
    // skip discriminator, message size, value_count
    rdr.skip(7);
    out.write(LEFT_BRACE)?;
    let mut first = true;
    for idx in 0..fields.len() {
        if first {
            first = false
        } else {
            out.write(COMMA)?;
        }

        let field = &fields[idx];
        out.write(DOUBLE_QUOTE)?;
        out.write(field.name.as_bytes())?;
        out.write(DOUBLE_QUOTE)?;
        out.write(COLON)?;

        let value_len = rdr.i32();
        if value_len == -1 {
            out.write(NULL)?;
        } else {
            let value_bytes = rdr.byte_slice(value_len as usize);
            write_value(value_bytes, &field.parser, out)?;
        }
    }
    out.write(RIGHT_BRACE)?;
    Ok(())
}

fn write_value<Out>(bytes: &[u8], parser: &Parser, out: &mut Out) -> Result<(), CasErr>
where
    Out: Write,
{
    match parser {
        Parser::Array => {
            let mut array = BinaryReader::from(bytes, ByteOrder::BigEndian);
            let ndims = array.i32();
            // something to do with nulls
            let _offset = array.i32();
            let item_oid = array.i32();
            let mut counts = Vec::new();
            for _ in 0..ndims {
                let vals_count = array.i32();
                counts.push(vals_count);
                // I haven't found a case where this doesn't equal 1.
                let _lower_bounds = array.i32();
            }
            let parser = find_parser(item_oid, &HashMap::new());
            out.write(LEFT_SQUARE)?;
            write_array_elements(&mut array, &counts, &parser, out)?;
            out.write(RIGHT_SQUARE)?;
        }
        Parser::BigNum => {
            let mut bignum = BinaryReader::from(bytes, ByteOrder::BigEndian);
            let digit_count = bignum.i16();
            let mut weight = bignum.i16();
            let flag = bignum.i16();
            let mut scale = bignum.i16();
            // If it’s zero, just write zero.
            if digit_count == 0 {
                out.write("0".as_bytes())?;
                return Ok(());
            }
            // TODO: Infinity and NaN
            // Add a negative sign if necessary.
            if flag == 0x4000 {
                out.write("-".as_bytes())?;
            }
            // Write integral part.
            if weight >= 0 {
                // Write the first block without leading zeros.
                let first_block = bignum.i16();
                serde_json::to_writer(&mut *out, &first_block)?;
                weight -= 1;
                // Write subsequent integral blocks as zero-padded.
                while weight >= 0 {
                    let block = bignum.i16();
                    write!(out, "{:04}", block)?;
                    weight -= 1;
                }
            } else {
                out.write("0".as_bytes())?;
            }
            // Write fractional part.
            if scale > 0 {
                out.write(".".as_bytes())?;
            } else {
                return Ok(())
            }
            // Add leading zeros if necessary, i.e. if the first digit block is more than 4 zeros
            // after the decimal. If we’ve just written an integral part, the weight will be -1.
            let zero_block_count = -1 - weight;
            for _ in 0..zero_block_count {
                out.write("0000".as_bytes())?;
                scale -= 4;
            }
            // Write blocks with leading and trailing zeros if present.
            while scale > 4 {
                let block = bignum.i16();
                write!(out, "{:04}", block)?;
                scale -= 4;
            }
            // Write final block, with leading but without trailing zeros.
            if scale > 0 {
                let block = bignum.i16();
                let digits = format!("{:04}", block);
                let trimmed = &digits.as_bytes()[0..(scale as usize)];
                out.write(trimmed)?;
            }
        }
        Parser::Bool => {
            let bool = bytes[0] == 1;
            serde_json::to_writer(out, &bool)?;
        }
        Parser::Bytes => {
            serde_json::to_writer(out, bytes)?;
        }
        Parser::Float32 => {
            let float = f32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
            serde_json::to_writer(out, &float)?;
        }
        Parser::Float64 => {
            let float = f64::from_be_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            ]);
            serde_json::to_writer(out, &float)?;
        }
        Parser::Int16 => {
            let int = i16::from_be_bytes([bytes[0], bytes[1]]);
            serde_json::to_writer(out, &int)?;
        }
        Parser::Int32 => {
            let int = i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
            serde_json::to_writer(out, &int)?;
        }
        Parser::Int64 => {
            let int = i64::from_be_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            ]);
            serde_json::to_writer(out, &int)?;
        }
        Parser::String => {
            serde_json::to_writer(out, std::str::from_utf8(bytes)?)?;
        }
        Parser::EWKB => {
            let geom = ewkb::parse_geom(bytes);
            serde_json::to_writer(out, &geom)?;
        }
        Parser::Tid => {
            // ( i32, i16 )
            let block = i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
            let offset = i16::from_be_bytes([bytes[4], bytes[5]]);
            out.write(LEFT_SQUARE)?;
            serde_json::to_writer(&mut (*out), &block)?;
            out.write(COMMA)?;
            serde_json::to_writer(&mut (*out), &offset)?;
            out.write(RIGHT_SQUARE)?;
        }
        Parser::Unknown => {
            serde_json::to_writer(out, "???")?;
        }
    }
    Ok(())
}

fn write_array_elements<Out>(
    bytes: &mut BinaryReader,
    dimensions: &[i32],
    parser: &Parser,
    out: &mut Out,
) -> Result<(), CasErr>
where
    Out: Write,
{
    if dimensions.len() == 1 {
        let mut first = true;
        for _ in 0..dimensions[0] {
            if first {
                first = false
            } else {
                out.write(COMMA)?;
            }
            let size = bytes.i32();
            if size == -1 {
                out.write(NULL)?;
            } else {
                write_value(bytes.byte_slice(size as usize), parser, out)?;
            }
        }
    } else {
        let mut first = true;
        for _ in 0..dimensions[0] {
            if first {
                first = false
            } else {
                out.write(COMMA)?;
            }
            out.write(LEFT_SQUARE)?;
            write_array_elements(bytes, &dimensions[1..dimensions.len()], parser, out)?;
            out.write(RIGHT_SQUARE)?;
        }
    }
    Ok(())
}

// https://github.com/postgres/postgres/blob/master/src/include/catalog/pg_type.dat
fn find_parser(oid: i32, dynamic_types: &HashMap<i32, String>) -> Parser {
    match oid {
        16 => Parser::Bool,     // bool
        17 => Parser::Bytes,    // bytea
        18 => Parser::String,   // char
        19 => Parser::String,   // name
        20 => Parser::Int64,    // int8
        21 => Parser::Int16,    // int2
        22 => Parser::Array,    // int2vector
        23 => Parser::Int32,    // int4
        24 => Parser::Int32,    // regproc (proc oid)
        25 => Parser::String,   // text
        26 => Parser::Int32,    // oid
        27 => Parser::Tid,      // tid
        28 => Parser::Int32,    // xid
        29 => Parser::Int32,    // cid
        30 => Parser::Array,    // oidvector
        194 => Parser::String,  // pg_node_tree (string representing an internal node tree)
        700 => Parser::Float32, // float4
        701 => Parser::Float64, // float8
        1007 => Parser::Array,  // int4[]
        1042 => Parser::String, // bpchar
        1043 => Parser::String, // varchar
        1700 => Parser::BigNum, // numeric
        _ => match dynamic_types.get(&oid).map(|typname| typname.as_str()) {
            Some("geometry") => Parser::EWKB,
            _ => Parser::Unknown,
        },
    }
}
