use crate::cas_err::CasErr;
use crate::postgres::bin_reader::{BinaryReader, ByteOrder};
use std::io::Write;

// TODO, this might end up as a class like JSONWriter, just to avoid sharing the args everywhere

enum EWKB {
    Point(Option<i32>),
    PointM(Option<i32>),
    PointZ(Option<i32>),
    PointMZ(Option<i32>),
}

pub fn write_geometry<T>(ewkb: &[u8], out: &mut T) -> Result<(), CasErr>
where
    T: Write,
{
    let order = match ewkb[0] {
        0 => ByteOrder::BigEndian,
        1 => ByteOrder::LittleEndian,
        _ => unreachable!(),
    };
    match ewkb[1] {
        1 => write_point(ewkb, order, out),
        _ => unimplemented!(),
    }
}

pub fn write_point<T>(ewkb: &[u8], order: ByteOrder, out: &mut T) -> Result<(), CasErr>
where
    T: Write,
{
    // let geom_type = r#"{"type":"Point","#;
    let mut reader = BinaryReader::from(ewkb, order);
    reader.skip(5); // skip order (u8) and point type (i32)

    // ST_AsGeoJSON will drop M if Z is also present, and if not, will make M the third coordinate.
    // It will only include the SRID if it’s not 4326, even if it’s explicitly set on the geometry.
    let point_type = match ewkb[4] {
        0x00 => EWKB::Point(None),
        0x20 => EWKB::Point(Some(reader.i32())),
        0x40 => EWKB::PointM(None),
        0x60 => EWKB::PointM(Some(reader.i32())),
        0x80 => EWKB::PointZ(None),
        0xA0 => EWKB::PointZ(Some(reader.i32())),
        0xC0 => EWKB::PointMZ(None),
        0xE0 => EWKB::PointMZ(Some(reader.i32())),
        _ => unreachable!(),
    };

    out.write(r#"{"type":"Point","#.as_bytes())?;
    // TODO: maybe this should be a single Point struct with a tag and srid field
    match point_type {
        EWKB::Point(Some(srid)) if srid != 4326 => {
            out.write(r#""crs":{"type":"name","properties":{"name":"EPSG:"#.as_bytes())?;
            // itoa::write(out, srid)?;
            out.write(srid.to_string().as_bytes())?; // mutability issues with itoa that would be solved by class
            out.write(r#""}},"#.as_bytes())?;
        }
        EWKB::PointM(Some(srid)) if srid != 4326 => {
            out.write(r#""crs":{"type":"name","properties":{"name":"EPSG:"#.as_bytes())?;
            // itoa::write(out, srid)?;
            out.write(srid.to_string().as_bytes())?; // mutability issues with itoa that would be solved by class
            out.write(r#""}},"#.as_bytes())?;
        }
        EWKB::PointZ(Some(srid)) if srid != 4326 => {
            out.write(r#""crs":{"type":"name","properties":{"name":"EPSG:"#.as_bytes())?;
            // itoa::write(out, srid)?;
            out.write(srid.to_string().as_bytes())?; // mutability issues with itoa that would be solved by class
            out.write(r#""}},"#.as_bytes())?;
        }
        EWKB::PointMZ(Some(srid)) if srid != 4326 => {
            out.write(r#""crs":{"type":"name","properties":{"name":"EPSG:"#.as_bytes())?;
            // itoa::write(out, srid)?;
            out.write(srid.to_string().as_bytes())?; // mutability issues with itoa that would be solved by class
            out.write(r#""}},"#.as_bytes())?;
        }
        _ => {}
    }
    out.write(r#""coordinates":["#.as_bytes())?;
    let mut buf = ryu::Buffer::new();
    match point_type {
        EWKB::Point(_) => {
            out.write(buf.format(reader.f64()).as_bytes())?;
            out.write(",".as_bytes())?;
            out.write(buf.format(reader.f64()).as_bytes())?;
        }
        EWKB::PointM(_) => {
            out.write(buf.format(reader.f64()).as_bytes())?;
            out.write(",".as_bytes())?;
            out.write(buf.format(reader.f64()).as_bytes())?;
            out.write(",".as_bytes())?;
            out.write(buf.format(reader.f64()).as_bytes())?;
        }
        EWKB::PointZ(_) => {
            out.write(buf.format(reader.f64()).as_bytes())?;
            out.write(",".as_bytes())?;
            out.write(buf.format(reader.f64()).as_bytes())?;
            out.write(",".as_bytes())?;
            out.write(buf.format(reader.f64()).as_bytes())?;
        }
        EWKB::PointMZ(_) => {
            out.write(buf.format(reader.f64()).as_bytes())?;
            out.write(",".as_bytes())?;
            out.write(buf.format(reader.f64()).as_bytes())?;
            out.write(",".as_bytes())?;
            out.write(buf.format(reader.f64()).as_bytes())?;
        }
    }
    out.write("]}".as_bytes())?;
    Ok(())
}
