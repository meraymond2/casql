// use crate::binary_reader::{BinaryReader, ByteOrder};
// use crate::postgres::postgis::geojson::Coords::*;
// use crate::postgres::postgis::geojson::Position::*;
// use crate::postgres::postgis::geojson::{GeoJSON, GeoJSONType, CRS};
//
// pub fn parse_geom(bytes: &[u8]) -> GeoJSON {
//     match bytes[1] {
//         0x01 => parse_point(bytes),
//         // 0x02 => parse_linestring(bytes),
//         _ => {
//             eprintln!("{:?}", bytes);
//             unimplemented!()
//         }
//     }
// }
//
// fn parse_point(bytes: &[u8]) -> GeoJSON {
//     let order = byte_order(bytes[0]);
//     let mut reader = BinaryReader::from(bytes, order);
//     // skip order (u8) and point type (i32)
//     reader.skip(5);
//
//     // TODO working, but should be refactored, maybe after lines or polys.
//     let srid = match bytes[4] {
//         0x00 => None,
//         0x20 => Some(reader.i32()),
//         0x40 => None,
//         0x60 => Some(reader.i32()),
//         0x80 => None,
//         0xA0 => Some(reader.i32()),
//         0xC0 => None,
//         0xE0 => Some(reader.i32()),
//         _ => unreachable!(),
//     };
//     let coordinates = match bytes[4] {
//         0x00 => Point(XY(reader.f64(), reader.f64())),
//         0x20 => Point(XY(reader.f64(), reader.f64())),
//         0x40 => Point(XYM(reader.f64(), reader.f64(), reader.f64())),
//         0x60 => Point(XYM(reader.f64(), reader.f64(), reader.f64())),
//         0x80 => Point(XYZ(reader.f64(), reader.f64(), reader.f64())),
//         0xA0 => Point(XYZ(reader.f64(), reader.f64(), reader.f64())),
//         0xC0 => Point(XYZM(reader.f64(), reader.f64(), reader.f64(), reader.f64())),
//         0xE0 => Point(XYZM(reader.f64(), reader.f64(), reader.f64(), reader.f64())),
//         _ => unreachable!(),
//     };
//     GeoJSON {
//         tag: GeoJSONType::Point,
//         crs: CRS::from(srid),
//         coordinates,
//     }
// }
//
// fn byte_order(first_byte: u8) -> ByteOrder {
//     match first_byte {
//         0x00 => ByteOrder::BigEndian,
//         0x01 => ByteOrder::LittleEndian,
//         _ => unreachable!(),
//     }
// }
