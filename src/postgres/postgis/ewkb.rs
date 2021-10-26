use crate::binary_reader::{BinaryReader, ByteOrder};
use crate::postgres::postgis::geojson::{GeoJSON, GeoJSONType, CRS};

// #[derive(Debug)]
// pub enum EWKB {
//     Point(Option<i32>, [f64; 2]),
//     PointM(Option<i32>, [f64; 3]),
//     PointZ(Option<i32>, [f64; 3]),
//     PointMZ(Option<i32>, [f64; 4]),
// }

// https://datatracker.ietf.org/doc/html/rfc7946

pub fn parse_geom(bytes: &[u8]) -> GeoJSON {
    match bytes[1] {
        0x01 => parse_point(bytes),
        // 0x02 => parse_linestring(bytes),
        _ => {
            eprintln!("{:?}", bytes);
            unimplemented!()
        }
    }
}

fn parse_point(bytes: &[u8]) -> GeoJSON {
    let order = byte_order(bytes[0]);
    let mut reader = BinaryReader::from(bytes, order);
    reader.skip(5); // skip order (u8) and point type (i32)
    let srid = match bytes[4] {
        0x00 => None,
        0x20 => Some(reader.i32()),
        0x40 => None,
        0x60 => Some(reader.i32()),
        0x80 => None,
        0xA0 => Some(reader.i32()),
        0xC0 => None,
        0xE0 => Some(reader.i32()),
        _ => unreachable!(),
    };
    GeoJSON {
        tag: GeoJSONType::Point,
        crs: CRS::from(srid),
    }
}
//
//
// fn parse_point(bytes: &[u8]) -> EWKB {
//     let order = byte_order(bytes[0]);
//     let mut reader = BinaryReader::from(bytes, order);
//     reader.skip(5); // skip order (u8) and point type (i32)
//     match bytes[4] {
//         0x00 => EWKB::Point(None, [reader.f64(), reader.f64()]),
//         0x20 => EWKB::Point(Some(reader.i32()), [reader.f64(), reader.f64()]),
//         0x40 => EWKB::PointM(None, [reader.f64(), reader.f64(), reader.f64()]),
//         0x60 => EWKB::PointM(
//             Some(reader.i32()),
//             [reader.f64(), reader.f64(), reader.f64()],
//         ),
//         0x80 => EWKB::PointZ(None, [reader.f64(), reader.f64(), reader.f64()]),
//         0xA0 => EWKB::PointZ(
//             Some(reader.i32()),
//             [reader.f64(), reader.f64(), reader.f64()],
//         ),
//         0xC0 => EWKB::PointMZ(
//             None,
//             [reader.f64(), reader.f64(), reader.f64(), reader.f64()],
//         ),
//         0xE0 => EWKB::PointMZ(
//             Some(reader.i32()),
//             [reader.f64(), reader.f64(), reader.f64(), reader.f64()],
//         ),
//         _ => unreachable!(),
//     }
// }

fn byte_order(first_byte: u8) -> ByteOrder {
    match first_byte {
        0x00 => ByteOrder::BigEndian,
        0x01 => ByteOrder::LittleEndian,
        _ => unreachable!(),
    }
}
