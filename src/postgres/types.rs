// use crate::postgres::backend_msgs::Field;
// use crate::postgres::postgis::ewkb;
// use std::collections::HashMap;
//
// pub type ParseClosure = Box<dyn FnMut(Option<&[u8]>, usize) -> (String, CasVal)>;
//
// pub fn parser_generator(fields: Vec<Field>, dynamic_types: HashMap<i32, String>) -> ParseClosure {
//     let f = move |maybe_bytes: Option<&[u8]>, idx: usize| {
//         let field: &Field = &fields[idx];
//         let parser = parser_for_oid(field.data_type_oid).or(dynamic_types
//             .get(&field.data_type_oid)
//             .and_then(parser_for_dynamic_type));
//         if let None = parser {
//             eprintln!("Unhandled oid {}.", field.data_type_oid);
//         }
//
//         if let Some(bytes) = maybe_bytes {
//             let val = match parser {
//                 Some(parser) => parse_value(bytes, parser),
//                 // None => CasVal::Unparsed("???"),
//                 None => [],
//             };
//             (field.name.clone(), val)
//         } else {
//             (field.name.clone(), CasVal::Null)
//         }
//     };
//     Box::new(f)
// }
//
// fn parse_value(bytes: &[u8], parser: Parser) -> &[u8] {
//     bytes
// }
//
// // fn parse_value(bytes: &[u8], parser: Parser) -> CasVal {
// //     match parser {
// //         Parser::Bool => {
// //             let bool = bytes[0] == 1;
// //             CasVal::Bool(bool)
// //         }
// //         Parser::Int16 => {
// //             let int = i16::from_be_bytes([bytes[0], bytes[1]]);
// //             CasVal::Int16(int)
// //         }
// //         Parser::Int32 => {
// //             let int = i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
// //             CasVal::Int32(int)
// //         }
// //         Parser::Int64 => {
// //             let int = i64::from_be_bytes([
// //                 bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
// //             ]);
// //             CasVal::Int64(int)
// //         }
// //         Parser::String => {
// //             let str = std::str::from_utf8(bytes).expect("Value will be a valid UTF-8 string.");
// //             CasVal::Str(str.to_owned())
// //         }
// //         Parser::EWKB => {
// //             let geom = ewkb::parse_geom(bytes);
// //             CasVal::Geom(geom)
// //         }
// //     }
// // }
//
// /// https://github.com/postgres/postgres/blob/master/src/include/catalog/pg_type.dat
// fn parser_for_oid(oid: i32) -> Option<Parser> {
//     match oid {
//         16 => Some(Parser::Bool),     // bool
//         18 => Some(Parser::String),   // char
//         19 => Some(Parser::String),   // name
//         20 => Some(Parser::Int64),    // int8
//         21 => Some(Parser::Int16),    // int2
//         23 => Some(Parser::Int32),    // int4
//         24 => Some(Parser::Int32),    // regproc (proc oid)
//         25 => Some(Parser::String),   // text
//         26 => Some(Parser::Int32),    // oid
//         194 => Some(Parser::String),  // pg_node_tree (string representing an internal node tree)
//         1043 => Some(Parser::String), // varchar
//         _ => None,
//     }
// }
//
// fn parser_for_dynamic_type(typname: &String) -> Option<Parser> {
//     match typname.as_str() {
//         "geometry" => Some(Parser::EWKB),
//         _ => None,
//     }
// }
//
// #[derive(Debug)]
// enum Parser {
//     Bool,
//     Int16,
//     Int32,
//     Int64,
//     String,
//     EWKB,
// }



// fn looop(counts: &[i32], msg: &mut BinaryReader, acc: &mut Vec<Option<i32>>) {
//     if counts.len() == 1 {
//         for _ in 0..counts[0] {
//             let size = msg.i32();
//             if size == -1 {
//                 acc.push(None);
//             } else {
//                 acc.push(Some(msg.i32()));
//             }
//         }
//     } else {
//         for i in 0..counts[0] {
//             looop(&counts[1..counts.len()], msg, acc)
//         }
//     }
// }
//
// fn parse_value(bytes: &[u8], parser: Parser) -> CasVal {
//     match parser {
//         Parser::Array => {
//             let mut msg = BinaryReader::from(bytes, ByteOrder::BigEndian);
//             let ndims = msg.i32();
//             // Something to do with Nulls
//             let _offset = msg.i32();
//             let item_oid = msg.i32();
//             let mut counts = Vec::new();
//             for _ in 0..ndims {
//                 let vals_count = msg.i32();
//                 counts.push(vals_count);
//                 // I haven't found a case where this doesn't equal 1.
//                 let _lower_bounds = msg.i32();
//                 // eprintln!(
//                 //     "> vals_count: {}\n> lower_bound: {}",
//                 //     vals_count, maybe_lower_bounds
//                 // );
//             }
//             let mut ints = Vec::new();
//             looop(&counts, &mut msg, &mut ints);
//             eprintln!("{:?}", ints);
//             CasVal::Bytes(Vec::new())
//         }
