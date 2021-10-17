
// This can be reused if I decide I want CSV writing.
pub enum Serialiser {
    Bool,
    Int16,
    Int32,
    String,
    Unknown,
}

// https://github.com/postgres/postgres/blob/master/src/include/catalog/pg_type.dat
pub fn oid_to_serialiser(oid: i32) -> Serialiser {
    match oid {
        16 => Serialiser::Bool, // bool
        18 => Serialiser::String, // char
        19 => Serialiser::String, // name
        21 => Serialiser::Int16, // int2
        23 => Serialiser::Int32, // int4
        24 => Serialiser::Int32, // regproc (proc oid)
        25 => Serialiser::String, // text
        26 => Serialiser::Int32, // oid
        194 => Serialiser::String, // pg_node_tree (string representing an internal node tree)
        _ => Serialiser::Unknown,
    }
}
// I'll do the rest later, I've only done enough to read pg_type for now.
