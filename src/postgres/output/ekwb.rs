use crate::binary_reader::{BinaryReader, ByteOrder};
use crate::cas_err::CasErr;
use std::io::Write;

const LEFT_SQUARE: &[u8] = "[".as_bytes();
const RIGHT_SQUARE: &[u8] = "]".as_bytes();
const COMMA: &[u8] = ",".as_bytes();

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
    let coord_size = match bytes[4] {
        0x00 => 2, // XY without SRID
        0x20 => 2, // XY with SRID
        0x40 => 3, // XYM without SRID
        0x60 => 3, // XYM with SRID
        0x80 => 3, // XYZ without SRID
        0xA0 => 3, // XYZ with SRID
        0xC0 => 4, // XYZM without SRID
        0xE0 => 4, // XYZM with SRID
        _ => unreachable!(),
    };
    // TODO: for now, just printing coords, add in the wrapper afterwards
    // eprintln!("{:?}", bytes);
    out.write(LEFT_SQUARE)?;
    match bytes[1] {
        1 => {
            write_coords(&mut rdr, &[coord_size], out)?;
        }
        2 => {
            let line_length = rdr.i32();
            write_coords(&mut rdr, &[line_length, coord_size], out)?;
        }
        3 => {
            // TODO: can I change write_coords to find the outer dimension length so I can use it
            // for polygons?
            let mut first = true;
            let line_count = rdr.i32();
            for _ in 0..line_count {
                if first {
                    first = false
                } else {
                    out.write(COMMA)?;
                }
                let line_length = rdr.i32();
                out.write(LEFT_SQUARE)?;
                write_coords(&mut rdr, &[line_length, coord_size], out)?;
                out.write(RIGHT_SQUARE)?;
            }
        }
        _ => {
            unimplemented!()
        }
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

fn write_coords<Out>(
    bytes: &mut BinaryReader,
    dimensions: &[i32],
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
            write_f64(bytes.f64(), out)?;
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
            write_coords(bytes, &dimensions[1..dimensions.len()], out)?;
            out.write(RIGHT_SQUARE)?;
        }
    }
    Ok(())
}

/// Like serialise_f64, but ignoring the possibility of NaNs and Infinities.
fn write_f64<Out>(float: f64, out: &mut Out) -> Result<(), CasErr>
where
    Out: Write,
{
    let mut ryu_buf = ryu::Buffer::new();
    let str = ryu_buf.format(float);
    out.write(str.as_bytes())?;
    Ok(())
}
