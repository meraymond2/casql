use crate::binary_reader::{BinaryReader, ByteOrder};

#[derive(Debug)]
pub enum EWKB {
    Point(Option<i32>, [f64; 2]),
    PointM(Option<i32>, [f64; 3]),
    PointZ(Option<i32>, [f64; 3]),
    PointMZ(Option<i32>, [f64; 4]),
}

pub fn parse_geom(bytes: &[u8]) -> EWKB {
    match bytes[1] {
        0x01 => parse_point(bytes),
        // 0x02 => parse_linestring(bytes),
        _ => {
            eprintln!("{:?}", bytes);
            unimplemented!()
        }
    }
}

fn parse_point(bytes: &[u8]) -> EWKB {
    let order = byte_order(bytes[0]);
    let mut reader = BinaryReader::from(bytes, order);
    reader.skip(5); // skip order (u8) and point type (i32)
    match bytes[4] {
        0x00 => EWKB::Point(None, [reader.f64(), reader.f64()]),
        0x20 => EWKB::Point(Some(reader.i32()), [reader.f64(), reader.f64()]),
        0x40 => EWKB::PointM(None, [reader.f64(), reader.f64(), reader.f64()]),
        0x60 => EWKB::PointM(
            Some(reader.i32()),
            [reader.f64(), reader.f64(), reader.f64()],
        ),
        0x80 => EWKB::PointZ(None, [reader.f64(), reader.f64(), reader.f64()]),
        0xA0 => EWKB::PointZ(
            Some(reader.i32()),
            [reader.f64(), reader.f64(), reader.f64()],
        ),
        0xC0 => EWKB::PointMZ(
            None,
            [reader.f64(), reader.f64(), reader.f64(), reader.f64()],
        ),
        0xE0 => EWKB::PointMZ(
            Some(reader.i32()),
            [reader.f64(), reader.f64(), reader.f64(), reader.f64()],
        ),
        _ => unreachable!(),
    }
}

fn byte_order(first_byte: u8) -> ByteOrder {
    match first_byte {
        0x00 => ByteOrder::BigEndian,
        0x01 => ByteOrder::LittleEndian,
        _ => unreachable!(),
    }
}
