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
            write_coords(&mut rdr, 1, coord_size, out)?;
        }
        2 => {
            write_coords(&mut rdr, 2, coord_size, out)?;
        }
        3 => {
            write_coords(&mut rdr, 3, coord_size, out)?;
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
    n_dims: i32,
    point_n_dims: i32,
    out: &mut Out,
) -> Result<(), CasErr>
where
    Out: Write,
{
    if n_dims == 1 {
        let mut first = true;
        for _ in 0..point_n_dims {
            if first {
                first = false
            } else {
                out.write(COMMA)?;
            }
            write!(out, "{}", bytes.f64())?;
        }
    } else {
        let mut first = true;
        // Unlike Postgres arrays, the sub-dimensions can be different lengths, so each line within
        // a polygon begins with its own length.
        let len = bytes.i32();
        for _ in 0..len {
            if first {
                first = false
            } else {
                out.write(COMMA)?;
            }
            out.write(LEFT_SQUARE)?;
            write_coords(bytes, n_dims - 1, point_n_dims, out)?;
            out.write(RIGHT_SQUARE)?;
        }
    }
    Ok(())
}
