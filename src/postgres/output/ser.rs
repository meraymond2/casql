use std::collections::HashMap;

#[derive(Debug)]
pub enum Ser {
    Array,
    Bool,
    BigNum,
    BitString,
    Bytes,
    Date,
    EWKB,
    Float32,
    Float64,
    Int16,
    Int32,
    Int64,
    Interval,
    Json,
    String,
    Tid,
    Timestamp,
    TimeUnzoned,
    TimeZoned,
    Unknown,
    Uuid,
}

// https://github.com/postgres/postgres/blob/master/src/include/catalog/pg_type.dat
pub fn find_serialiser(oid: i32, dynamic_types: &HashMap<i32, String>) -> Ser {
    // eprintln!("{:?}", oid);
    match oid {
        16 => Ser::Bool,          // bool
        17 => Ser::Bytes,         // bytea
        18 => Ser::String,        // char
        19 => Ser::String,        // name
        20 => Ser::Int64,         // int8
        21 => Ser::Int16,         // int2
        22 => Ser::Array,         // int2vector
        23 => Ser::Int32,         // int4
        24 => Ser::Int32,         // regproc (proc oid)
        25 => Ser::String,        // text
        26 => Ser::Int32,         // oid
        27 => Ser::Tid,           // tid
        28 => Ser::Int32,         // xid
        29 => Ser::Int32,         // cid
        30 => Ser::Array,         // oidvector
        114 => Ser::Json,         // json
        142 => Ser::String,       // xml
        194 => Ser::String,       // pg_node_tree (string representing an internal node tree)
        700 => Ser::Float32,      // float4
        701 => Ser::Float64,      // float8
        1007 => Ser::Array,       // int4[]
        1042 => Ser::String,      // bpchar
        1043 => Ser::String,      // varchar
        1082 => Ser::Date,        // date
        1083 => Ser::TimeUnzoned, // time
        1114 => Ser::Timestamp,   // timestamp
        1184 => Ser::Timestamp,   // timestamptz
        1186 => Ser::Interval,    // interval
        1266 => Ser::TimeZoned,   // timetz
        1560 => Ser::BitString,   // bit
        1562 => Ser::BitString,   // varbit
        1700 => Ser::BigNum,      // numeric
        2950 => Ser::Uuid,        // uuid
        3802 => Ser::Json,        // jsonb
        4072 => Ser::String,      // jsonpath
        _ => match dynamic_types.get(&oid).map(|typname| typname.as_str()) {
            Some("geometry") => Ser::EWKB,
            _ => Ser::Unknown,
        },
    }
}
