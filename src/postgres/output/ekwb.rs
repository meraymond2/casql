use crate::binary_reader::{BinaryReader, ByteOrder};
use crate::cas_err::CasErr;
use std::io::Write;

const LEFT_SQUARE: &[u8] = "[".as_bytes();
const RIGHT_SQUARE: &[u8] = "]".as_bytes();
const COMMA: &[u8] = ",".as_bytes();

/// Given:
/// u8: byte order
/// i32: the least significant byte is the geometry type, the most is the coordinate type, the
///      middle two bytes appear to always be empty
/// Option<i32>: an SRID if it was specified at insert time
/// f64[]: coordinates, TODO: document the formats by type
///
/// Writes:
/// A Geojson object. If an SRID exists it will always be included, unlike ST_AsGeoJson, which skips
/// it if it’s 4326. Also, it includes M coordinates if present.
///
pub fn serialise_geom<Out>(bytes: &[u8], out: &mut Out) -> Result<(), CasErr>
where
    Out: Write,
{
    let byte_order = if bytes[0] == 0 {
        ByteOrder::BigEndian
    } else {
        ByteOrder::LittleEndian
    };
    let mut rdr = BinaryReader::from(bytes, byte_order);
    // skip byte_order (u8)
    rdr.skip(1);
    let geom_type = rdr.u8();
    rdr.skip(2);
    let coord_type = rdr.u8();
    let srid = if has_srid(coord_type) {
        Some(rdr.i32())
    } else {
        None
    };
    let coord_dims = coord_size(coord_type);
    write!(out, "{{\"type\":\"{}\",", geom_name(geom_type))?;
    if let Some(srid) = srid {
        write!(out, "\"crs\":{{\"type\":\"name\",\"properties\":{{\"name\":\"EPSG:{}\"}}}},", srid)?;
    }
    match geom_type {
        1 => {
            // Point
            write!(out, "\"coordinates\":")?;
            write_coords(&mut rdr, 1, coord_dims, out)?;
        }
        2 => {
            // Linestring
            write!(out, "\"coordinates\":")?;
            write_coords(&mut rdr, 2, coord_dims, out)?;
        }
        3 => {
            // Polygon
            write!(out, "\"coordinates\":")?;
            write_coords(&mut rdr, 3, coord_dims, out)?;
        }
        4 => {
            // Multipoint
            write!(out, "\"coordinates\":")?;
            write_coords_array(&mut rdr, out)?;
        }
        5 => {
            // Multilinestring
            write!(out, "\"coordinates\":")?;
            write_coords_array(&mut rdr, out)?;
        }
        6 => {
            // Multipolygon
            write!(out, "\"coordinates\":")?;
            write_coords_array(&mut rdr, out)?;
        }
        7 => {
            // Geometry Collection {
            write!(out, "\"geometries\":")?;
            write_collection(&mut rdr, out)?;
        }
        _ => {
            unimplemented!("{}", geom_type)
        }
    }
    write!(out, "}}")?;
    Ok(())
}

fn geom_name(geom_type: u8) -> &'static str {
    match geom_type {
        1 => "Point",
        2 => "LineString",
        3 => "Polygon",
        4 => "MultiPoint",
        5 => "MultiLineString",
        6 => "MultiPolygon",
        7 => "GeometryCollection",
        _ => unimplemented!("{}", geom_type)
    }
}

fn write_coords_array<Out>(rdr: &mut BinaryReader, out: &mut Out) -> Result<(), CasErr>
where
    Out: Write,
{
    let collection_size = rdr.i32();
    let mut first = true;
    out.write(LEFT_SQUARE)?;
    for _ in 0..collection_size {
        if first {
            first = false;
        } else {
            out.write(COMMA)?;
        }
        // Not sure what his flag is, it has always been 1 so far.
        let _flag = rdr.u8();
        let geom_type = rdr.u8();
        rdr.skip(2);
        let coord_type = rdr.u8();
        let n_dims = geom_type; // Point (1) -> 1, Line (2) -> 2, etc.
        write_coords(rdr, n_dims as i32, coord_size(coord_type), out)?;
    }
    out.write(RIGHT_SQUARE)?;
    Ok(())
}

fn write_coords<Out>(
    bytes: &mut BinaryReader,
    n_dims: i32,
    point_n_dims: usize,
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

fn write_collection<Out>(_bytes: &mut BinaryReader, _out: &mut Out) -> Result<(), CasErr>
    where
        Out: Write,
{
   todo!()
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

fn coord_size(flag: u8) -> usize {
    match flag {
        0x00 => 2, // XY without SRID
        0x20 => 2, // XY with SRID
        0x40 => 3, // XYM without SRID
        0x60 => 3, // XYM with SRID
        0x80 => 3, // XYZ without SRID
        0xA0 => 3, // XYZ with SRID
        0xC0 => 4, // XYZM without SRID
        0xE0 => 4, // XYZM with SRID
        _ => unreachable!(),
    }
}
