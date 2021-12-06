use crate::binary_reader::{BinaryReader, ByteOrder};
use crate::postgres::postgis::ewkb;
use crate::postgres::row_iter::RowIter;
use crate::CasErr;
use std::collections::HashMap;
use std::io::{BufWriter, Write};

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
    Int16,
    Int32,
    Int64,
    String,
    EWKB,
    Unknown,
}

struct JsonField {
    name: String,
    parser: Parser,
}

pub fn write_rows(rows: RowIter, dynamic_types: &HashMap<i32, String>) -> Result<(), CasErr> {
    let stdout = std::io::stdout();
    let handle = stdout.lock();
    let mut out = BufWriter::new(handle);
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
        write_row(row, &fields, &mut out)?;
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
        Parser::Bool => {
            let bool = bytes[0] == 1;
            serde_json::to_writer(out, &bool)?;
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
        Parser::Unknown => {}
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

fn find_parser(oid: i32, dynamic_types: &HashMap<i32, String>) -> Parser {
    match oid {
        16 => Parser::Bool,     // bool
        18 => Parser::String,   // char
        19 => Parser::String,   // name
        20 => Parser::Int64,    // int8
        21 => Parser::Int16,    // int2
        23 => Parser::Int32,    // int4
        24 => Parser::Int32,    // regproc (proc oid)
        25 => Parser::String,   // text
        26 => Parser::Int32,    // oid
        194 => Parser::String,  // pg_node_tree (string representing an internal node tree)
        1007 => Parser::Array,  // int4[]
        1043 => Parser::String, // varchar
        _ => match dynamic_types.get(&oid).map(|typname| typname.as_str()) {
            Some("geometry") => Parser::EWKB,
            _ => Parser::Unknown,
        },
    }
}