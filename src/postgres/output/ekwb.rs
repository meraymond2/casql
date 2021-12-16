use crate::binary_reader::{BinaryReader, ByteOrder};
use crate::cas_err::CasErr;
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

/// Given:
///
/// Writes:
///
pub fn serialise_geom<Out>(bytes: &[u8], out: &mut Out) -> Result<(), CasErr>
where
    Out: Write,
{
    let order = byte_order(bytes[0]);
    let mut rdr = BinaryReader::from(bytes, order);
    // skip order (u8)
    rdr.skip(1);
    let geom_type = rdr.i32();
    // todo is it always the fifth byte?
    let srid = if has_srid(bytes[4]) {
        Some(rdr.i32())
    } else {
        None
    };
    match bytes[1] {
        1 => write_point_coords(bytes[4], &mut rdr, out),
        // 2 => parse_linestring(bytes),
        _ => {
            unimplemented!()
        }
    }
}

pub fn write_point_coords<Out>(flag: u8, bytes: &mut BinaryReader, out: &mut Out) -> Result<(), CasErr>
where
    Out: Write,
{
    // just temporary output
    out.write(LEFT_SQUARE)?;
    match flag {
        0x00 => {
            // XY without SRID
            let mut ryu_buf = ryu::Buffer::new();
            let str = ryu_buf.format(bytes.f64());
            out.write(str.as_bytes())?;
            out.write(COMMA)?;
            let str = ryu_buf.format(bytes.f64());
            out.write(str.as_bytes())?;
        }
        0x20 => {
            // XY with SRID
            let mut ryu_buf = ryu::Buffer::new();
            let str = ryu_buf.format(bytes.f64());
            out.write(str.as_bytes())?;
            out.write(COMMA)?;
            let str = ryu_buf.format(bytes.f64());
            out.write(str.as_bytes())?;
        }
        0x40 => {
            // XYM without SRID
            let mut ryu_buf = ryu::Buffer::new();
            let str = ryu_buf.format(bytes.f64());
            out.write(str.as_bytes())?;
            out.write(COMMA)?;
            let str = ryu_buf.format(bytes.f64());
            out.write(str.as_bytes())?;
            out.write(COMMA)?;
            let str = ryu_buf.format(bytes.f64());
            out.write(str.as_bytes())?;
        },
        0x60 => {
            // XYM with SRID
            let mut ryu_buf = ryu::Buffer::new();
            let str = ryu_buf.format(bytes.f64());
            out.write(str.as_bytes())?;
            out.write(COMMA)?;
            let str = ryu_buf.format(bytes.f64());
            out.write(str.as_bytes())?;
            out.write(COMMA)?;
            let str = ryu_buf.format(bytes.f64());
            out.write(str.as_bytes())?;
        },
        0x80 => {
            // XYZ without SRID
            let mut ryu_buf = ryu::Buffer::new();
            let str = ryu_buf.format(bytes.f64());
            out.write(str.as_bytes())?;
            out.write(COMMA)?;
            let str = ryu_buf.format(bytes.f64());
            out.write(str.as_bytes())?;
            out.write(COMMA)?;
            let str = ryu_buf.format(bytes.f64());
            out.write(str.as_bytes())?;
        },
        0xA0 => {
            // XYZ with SRID
            let mut ryu_buf = ryu::Buffer::new();
            let str = ryu_buf.format(bytes.f64());
            out.write(str.as_bytes())?;
            out.write(COMMA)?;
            let str = ryu_buf.format(bytes.f64());
            out.write(str.as_bytes())?;
            out.write(COMMA)?;
            let str = ryu_buf.format(bytes.f64());
            out.write(str.as_bytes())?;
        },
        0xC0 => {
            // XYZM without SRID
            let mut ryu_buf = ryu::Buffer::new();
            let str = ryu_buf.format(bytes.f64());
            out.write(str.as_bytes())?;
            out.write(COMMA)?;
            let str = ryu_buf.format(bytes.f64());
            out.write(str.as_bytes())?;
            out.write(COMMA)?;
            let str = ryu_buf.format(bytes.f64());
            out.write(str.as_bytes())?;
            out.write(COMMA)?;
            let str = ryu_buf.format(bytes.f64());
            out.write(str.as_bytes())?;
        },
        0xE0 => {
            // XYZM with SRID
            let mut ryu_buf = ryu::Buffer::new();
            let str = ryu_buf.format(bytes.f64());
            out.write(str.as_bytes())?;
            out.write(COMMA)?;
            let str = ryu_buf.format(bytes.f64());
            out.write(str.as_bytes())?;
            out.write(COMMA)?;
            let str = ryu_buf.format(bytes.f64());
            out.write(str.as_bytes())?;
            out.write(COMMA)?;
            let str = ryu_buf.format(bytes.f64());
            out.write(str.as_bytes())?;
        },
        _ => unreachable!(),
    }
    out.write(RIGHT_SQUARE)?;
    Ok(())
}

fn byte_order(flag: u8) -> ByteOrder {
    if flag == 0 {
        ByteOrder::BigEndian
    } else {
        // first_byte == 1
        ByteOrder::LittleEndian
    }
}

fn has_srid(flag: u8) -> bool {
    match flag {
        0x00 => false,
        0x20 => true,
        0x40 => false,
        0x60 => true,
        0x80 => false,
        0xA0 => true,
        0xC0 => false,
        0xE0 => true,
        _ => unreachable!(),
    }
}

// TODO: use already defined one
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
    } else {
        let mut first = true;
        for _ in 0..dimensions[0] {
            if first {
                first = false
            } else {
                out.write(COMMA)?;
            }
            out.write(LEFT_SQUARE)?;
            write_array_elements(bytes, &dimensions[1..dimensions.len()], serialiser, out)?;
            out.write(RIGHT_SQUARE)?;
        }
    }
    Ok(())
}
