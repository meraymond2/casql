use crate::postgres::row_iter::CasVal;
use std::collections::HashMap;

pub fn parse_value(value: &[u8], oid: i32, dynamic_types: &HashMap<i32, String>) -> CasVal {
    let parser = built_in_types(oid).or_else(|| dynamic_types.get(&oid).and_then(runtime_types));
    // TODO: can I build a parser function instead of looking it up for every val? I could do it once built on the fields and the dynamic types.
    println!("{:?}", parser);
    CasVal::Null
}

fn built_in_types(oid: i32) -> Option<Parser> {
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

fn runtime_types(typname: &String) -> Option<Parser> {
    match typname.as_str() {
        "geometry" => Some(Parser::EWKB),
        _ => None,
    }
}

#[derive(Debug)]
enum Parser {
    Bool,
    Int16,
    Int32,
    String,
    EWKB,
}
