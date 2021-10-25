use crate::postgres::backend_msgs::Field;
use crate::cas_val::CasVal;

pub type ParseClosure = Box<dyn FnMut(Option<&[u8]>, usize) -> (String, CasVal)>;

pub fn parser_generator(fields: Vec<Field>) -> ParseClosure {
    let f = move |maybe_bytes: Option<&[u8]>, idx: usize| {
        let field: &Field = &fields[idx];
        let parser = parser_for_oid(field.data_type_oid);
        if let Some(bytes) = maybe_bytes {
            let val = match parser {
                Some(parser) => parse_value(bytes, parser),
                None => CasVal::Unparsed,
            };
            (field.name.clone(), val)
        } else {
            (field.name.clone(), CasVal::Null)
        }
    };
    Box::new(f)
}

fn parse_value(bytes: &[u8], parser: Parser) -> CasVal {
    match parser {
        Parser::Bool => {
            let bool = bytes[0] == 1;
            CasVal::Bool(bool)
        }
        Parser::Int16 => {
            let int = i16::from_be_bytes([bytes[0], bytes[1]]);
            CasVal::Int16(int)
        }
        Parser::Int32 => {
            let int = i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
            CasVal::Int32(int)
        }
        Parser::String => {
            let str = std::str::from_utf8(bytes).expect("Value will be a valid UTF-8 string.");
            CasVal::Str(str.to_owned()) // TODO: make CasVal::Str take a reference, might need to replace iterator
        }
        Parser::EWKB => {
            unimplemented!()
        }
    }
}

// use crate::postgres::row_iter::CasVal;
// use std::collections::HashMap;
//
// pub fn parse_value(value: &[u8], oid: i32, dynamic_types: &HashMap<i32, String>) -> CasVal {
//     let parser = built_in_types(oid).or_else(|| dynamic_types.get(&oid).and_then(runtime_types));
//     // TODO: can I build a parser function instead of looking it up for every val? I could do it once built on the fields and the dynamic types.
//     println!("{:?}", parser);
//     CasVal::Null
// }
//
fn parser_for_oid(oid: i32) -> Option<Parser> {
    match oid {
        16 => Some(Parser::Bool),    // bool
        18 => Some(Parser::String),  // char
        19 => Some(Parser::String),  // name
        21 => Some(Parser::Int16),   // int2
        23 => Some(Parser::Int32),   // int4
        24 => Some(Parser::Int32),   // regproc (proc oid)
        25 => Some(Parser::String),  // text
        26 => Some(Parser::Int32),   // oid
        194 => Some(Parser::String), // pg_node_tree (string representing an internal node tree)
        _ => None,
    }
}

// fn runtime_types(typname: &String) -> Option<Parser> {
//     match typname.as_str() {
//         "geometry" => Some(Parser::EWKB),
//         _ => None,
//     }
// }

#[derive(Debug)]
enum Parser {
    Bool,
    Int16,
    Int32,
    String,
    EWKB,
}
