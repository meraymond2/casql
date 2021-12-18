use crate::binary_reader::{BinaryReader, ByteOrder};
use crate::cas_err::CasErr;
use crate::postgres::output::ser::{find_serialiser, Ser};
use crate::postgres::output::{binary, ekwb, json, nums, shapes, text, time};
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

struct JsonField {
    name: String,
    serialiser: Ser,
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
            serialiser: find_serialiser(field.data_type_oid, dynamic_types),
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
            write_value(value_bytes, &field.serialiser, out)?;
        }
    }
    out.write(RIGHT_BRACE)?;
    Ok(())
}

fn write_value<Out>(bytes: &[u8], serialiser: &Ser, out: &mut Out) -> Result<(), CasErr>
where
    Out: Write,
{
    match serialiser {
        Ser::Array => write_array(bytes, out),
        Ser::Bool => nums::serialise_bool(bytes, out),
        Ser::BigNum => nums::serialise_bignum(bytes, out),
        Ser::BitString => binary::serialise_bitstring(bytes, out),
        Ser::Box => shapes::serialise_box(bytes, out),
        Ser::Bytes => binary::serialise_bytes(bytes, out),
        Ser::Circle => shapes::serialise_circle(bytes, out),
        Ser::Date => time::serialise_date(bytes, out),
        Ser::EWKB => ekwb::serialise_geom(bytes, out),
        Ser::Float32 => nums::serialise_f32(bytes, out),
        Ser::Float64 => nums::serialise_f64(bytes, out),
        Ser::Int16 => nums::serialise_i16(bytes, out),
        Ser::Int32 => nums::serialise_i32(bytes, out),
        Ser::Int64 => nums::serialise_i64(bytes, out),
        Ser::Interval => time::serialise_duration(bytes, out),
        Ser::Json => json::serialise_json(bytes, out),
        Ser::Line => shapes::serialise_line(bytes, out),
        Ser::LineSegment => shapes::serialise_line_segment(bytes, out),
        Ser::Path => shapes::serialise_path(bytes, out),
        Ser::Point => shapes::serialise_point(bytes, out),
        Ser::Polygon => shapes::serialise_polygon(bytes, out),
        Ser::String => text::serialise_str(bytes, out),
        Ser::Tid => nums::serialise_tid(bytes, out),
        Ser::Timestamp => time::serialise_datetime(bytes, out),
        Ser::TimeUnzoned => time::serialise_time_unzoned(bytes, out),
        Ser::TimeZoned => time::serialise_time_zoned(bytes, out),
        Ser::Unknown => {
            out.write("???".as_bytes())?;
            Ok(())
        }
        Ser::Uuid => binary::serialise_uuid(bytes, out),
    }
}

fn write_array<Out>(bytes: &[u8], out: &mut Out) -> Result<(), CasErr>
where
    Out: Write,
{
    let mut array = BinaryReader::from(bytes, ByteOrder::BigEndian);
    let n_dims = array.i32();
    // something to do with nulls
    let _offset = array.i32();
    let item_oid = array.i32();
    let mut counts = Vec::new();
    for _ in 0..n_dims {
        let vals_count = array.i32();
        counts.push(vals_count);
        // I haven't found a case where this doesn't equal 1.
        let _lower_bounds = array.i32();
    }
    let parser = find_serialiser(item_oid, &HashMap::new());
    write_array_elements(&mut array, &counts, &parser, out)?;
    Ok(())
}

fn write_array_elements<Out>(
    bytes: &mut BinaryReader,
    dimensions: &[i32],
    serialiser: &Ser,
    out: &mut Out,
) -> Result<(), CasErr>
where
    Out: Write,
{
    if dimensions.len() == 1 {
        let mut first = true;
        out.write(LEFT_SQUARE)?;
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
                write_value(bytes.byte_slice(size as usize), serialiser, out)?;
            }
        }
        out.write(RIGHT_SQUARE)?;
    } else {
        let mut first = true;
        out.write(LEFT_SQUARE)?;
        for _ in 0..dimensions[0] {
            if first {
                first = false
            } else {
                out.write(COMMA)?;
            }
            write_array_elements(bytes, &dimensions[1..dimensions.len()], serialiser, out)?;
        }
        out.write(RIGHT_SQUARE)?;
    }
    Ok(())
}
