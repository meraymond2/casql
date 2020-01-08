use mysql;
use serde::ser::{Serialize, Serializer};
use chrono::{DateTime, Utc, Local};

#[derive(Clone, Debug)]
pub enum CasVal {
  Str(String),
  UUID(String),
  Bool(bool),
  UtcDate(DateTime<Utc>),
  LocalDate(DateTime<Local>),
  Int(i64),
  Uint(u64),
  Float(f64),
  Json(serde_json::Value),
  Null,
  Unknown,
}

impl Serialize for CasVal {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    match self {
      CasVal::Str(string) => serializer.serialize_str(string),
      CasVal::UUID(string) => serializer.serialize_str(string),
      CasVal::Null => serializer.serialize_unit(),
      CasVal::Bool(b) => serializer.serialize_bool(*b),
      CasVal::Int(n) => serializer.serialize_i64(*n),
      CasVal::Uint(n) => serializer.serialize_u64(*n),
      CasVal::Float(n) => serializer.serialize_f64(*n),
      CasVal::UtcDate(date) => serializer.serialize_str(&date.to_string()),
      CasVal::LocalDate(date) => serializer.serialize_str(&date.to_string()),
      CasVal::Json(json) => serializer.serialize_str(&json.to_string()),
      CasVal::Unknown => serializer.serialize_str("???"),
    }
  }
}

impl From<mysql::Value> for CasVal {
  fn from(my_val: mysql::Value) -> Self {
    match my_val {
      mysql::Value::NULL => CasVal::Null,
      mysql::Value::Bytes(bytes) => CasVal::Str(String::from_utf8(bytes).unwrap()),
      mysql::Value::Int(i) => CasVal::Int(i),
      mysql::Value::UInt(u) => CasVal::Uint(u),
      mysql::Value::Float(f) => CasVal::Float(f),
      // year, month, day, hour, minutes, seconds, micro seconds
      // mysql::Value::Date(u16, u8, u8, u8, u8, u8, u32)
      // mysql::Value::Time(bool, u32, u8, u8, u8, u32)
      _ => CasVal::Unknown,
    }
  }
}
