use serde::{Serialize, Serializer};

#[derive(Debug)]
pub enum CasVal {
    Bool(bool),
    Int16(i16),
    Int32(i32),
    Str(String),
    Null,
    Unparsed,
}

impl Serialize for CasVal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            CasVal::Bool(b) => serializer.serialize_bool(*b),
            CasVal::Null => serializer.serialize_unit(),
            CasVal::Int16(int) => serializer.serialize_i16(*int),
            CasVal::Int32(int) => serializer.serialize_i32(*int),
            CasVal::Str(string) => serializer.serialize_str(string),
            CasVal::Unparsed => serializer.serialize_str("???"),


            // CasVal::UUID(string) => serializer.serialize_str(string),
            // CasVal::Int64(n) => serializer.serialize_i64(*n),
            // CasVal::UInt32(n) => serializer.serialize_u32(*n),
            // CasVal::UInt64(n) => serializer.serialize_u64(*n),
            // CasVal::Float32(n) => serializer.serialize_f32(*n),
            // CasVal::Float64(n) => serializer.serialize_f64(*n),
            // CasVal::UtcDate(date) => serializer.serialize_str(&date.to_string()),
            // CasVal::LocalDate(date) => serializer.serialize_str(&date.to_string()),
            // CasVal::Json(json) => serializer.serialize_str(&json.to_string()),
            // CasVal::Unknown => serializer.serialize_str("???"),
        }
    }
}
