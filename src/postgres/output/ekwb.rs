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
    let _geom_type = rdr.i32();
    // todo is it always the fifth byte?
    let _srid = if has_srid(bytes[4]) {
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
            // Point
            write_coords(&mut rdr, 1, coord_size, out)?;
        }
        2 => {
            // Linestring
            write_coords(&mut rdr, 2, coord_size, out)?;
        }
        3 => {
            // Polygon
            write_coords(&mut rdr, 3, coord_size, out)?;
        }
        4 => {
            // Multipoint
            write_collection_coords(&mut rdr, 1, coord_size, out)?;
        }
        5 => {
            // Multilinestring
            write_collection_coords(&mut rdr, 2, coord_size, out)?;
        }
        6 => {
            // Multipolygon
            write_collection_coords(&mut rdr, 3, coord_size, out)?;
        }
        7 => {
            // Geometry Collection {
            eprintln!("{:?}", bytes);
            write_collection_coords_alt(&mut rdr, out)?;
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

fn write_collection_coords_alt<Out>(
    bytes: &mut BinaryReader,
    out: &mut Out,
) -> Result<(), CasErr>
    where
        Out: Write,
{
    let collection_size = bytes.i32();
    eprintln!("coll size {}", collection_size);
    let mut first = true;
    for _ in 0..collection_size {
        if first {
            first = false;
        } else {
            out.write(COMMA)?;
        }
        // what is this and is it ever not one?
        let _flag = bytes.u8();
        eprintln!("flag {}", _flag);
        // This is the geometry type and coordinate size for the elements within the collection,
        // but it is derivable from the collection’s type, so rather than repeat that match
        // statement, we ignore it here. I haven’t come across a case yet where you can mix
        // coordinate-types, this assumption may change if that is possible.
        let _coll_geom_type = bytes.i32();
        eprintln!("ach {:?}", _coll_geom_type.to_le_bytes());
        let n_dims = match _coll_geom_type.to_le_bytes()[0] {
            1 => 1,
            2 => 2,
            3 => 3,
            4 => 1,
            5 => 2,
            6 => 3,
            _ => unreachable!()
        };
        let coord_size = match _coll_geom_type.to_le_bytes()[3] {
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
        write_coords(bytes, n_dims, coord_size, out)?;
    }
    Ok(())
}

fn write_collection_coords<Out>(
    bytes: &mut BinaryReader,
    n_dims: i32,
    coord_size: i32,
    out: &mut Out,
) -> Result<(), CasErr>
where
    Out: Write,
{
    let collection_size = bytes.i32();
    let mut first = true;
    for _ in 0..collection_size {
        if first {
            first = false;
        } else {
            out.write(COMMA)?;
        }
        // what is this and is it ever not one?
        let _flag = bytes.u8();
        // This is the geometry type and coordinate size for the elements within the collection,
        // but it is derivable from the collection’s type, so rather than repeat that match
        // statement, we ignore it here. I haven’t come across a case yet where you can mix
        // coordinate-types, this assumption may change if that is possible.
        let _coll_geom_type = bytes.i32();
        write_coords(bytes, n_dims, coord_size, out)?;
    }
    Ok(())
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
        out.write(LEFT_SQUARE)?;
        for _ in 0..point_n_dims {
            if first {
                first = false
            } else {
                out.write(COMMA)?;
            }
            write!(out, "{}", bytes.f64())?;
        }
        out.write(RIGHT_SQUARE)?;
    } else {
        let mut first = true;
        // Unlike Postgres arrays, the sub-dimensions can be different lengths, so each line within
        // a polygon begins with its own length.
        let len = bytes.i32();
        out.write(LEFT_SQUARE)?;
        for _ in 0..len {
            if first {
                first = false
            } else {
                out.write(COMMA)?;
            }
            write_coords(bytes, n_dims - 1, point_n_dims, out)?;
        }
        out.write(RIGHT_SQUARE)?;
    }
    Ok(())
}
