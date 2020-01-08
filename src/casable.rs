use chrono::{DateTime, Local, Utc};
use mysql;
use mysql::consts::ColumnType;
use serde::ser::{Serialize, Serializer};

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

pub fn from_mysql_value(my_val: mysql::Value, ty: mysql::consts::ColumnType) -> CasVal {
  match my_val {
    mysql::Value::NULL => CasVal::Null,
    mysql::Value::Bytes(bytes) => {
      let s = String::from_utf8(bytes).unwrap();
      match ty {
        ColumnType::MYSQL_TYPE_LONG => CasVal::Int(i64::from_str_radix(&s, 10).unwrap()),
        // ColumnType::MYSQL_TYPE_DECIMAL
        // ColumnType::MYSQL_TYPE_TINY
        // ColumnType::MYSQL_TYPE_SHORT
        // ColumnType::MYSQL_TYPE_LONG
        // ColumnType::MYSQL_TYPE_FLOAT
        // ColumnType::MYSQL_TYPE_DOUBLE
        // ColumnType::MYSQL_TYPE_NULL
        ColumnType::MYSQL_TYPE_TIMESTAMP => {
          // TODO: Convert to UTC Datetime, or TZ, whichever this represents
          // the string looks like "2020-01-08 22:00:14"
          CasVal::Str(s)
        },
        // ColumnType::MYSQL_TYPE_LONGLONG
        // ColumnType::MYSQL_TYPE_INT24
        // ColumnType::MYSQL_TYPE_DATE
        // ColumnType::MYSQL_TYPE_TIME
        ColumnType::MYSQL_TYPE_DATETIME => {
          // TODO: Convert to UTC Datetime, or TZ, whichever this represents
          // the string looks like "2020-01-08 22:00:14"
          CasVal::Str(s)
        },
        // ColumnType::MYSQL_TYPE_YEAR
        // ColumnType::MYSQL_TYPE_NEWDATE
        ColumnType::MYSQL_TYPE_VARCHAR => CasVal::Str(s),
        // ColumnType::MYSQL_TYPE_BIT
        // ColumnType::MYSQL_TYPE_TIMESTAMP2
        // ColumnType::MYSQL_TYPE_DATETIME2
        // ColumnType::MYSQL_TYPE_TIME2
        // ColumnType::MYSQL_TYPE_JSON
        // ColumnType::MYSQL_TYPE_NEWDECIMAL
        // ColumnType::MYSQL_TYPE_ENUM
        // ColumnType::MYSQL_TYPE_SET
        // ColumnType::MYSQL_TYPE_TINY_BLOB
        // ColumnType::MYSQL_TYPE_MEDIUM_BLOB
        // ColumnType::MYSQL_TYPE_LONG_BLOB
        ColumnType::MYSQL_TYPE_BLOB => {
          // What's a blob???
          CasVal::Str(s)
        },
        ColumnType::MYSQL_TYPE_VAR_STRING => CasVal::Str(s),
        ColumnType::MYSQL_TYPE_STRING => CasVal::Str(s),
        // ColumnType::MYSQL_TYPE_GEOMETRY
        _ => {
          eprintln!("As yet unsupported MySQL type: {:?}", ty);
          CasVal::Str("Cascat!!!!".to_owned())
        }
          ,
      }
    }
    mysql::Value::Int(i) => CasVal::Int(i),
    mysql::Value::UInt(u) => CasVal::Uint(u),
    mysql::Value::Float(f) => CasVal::Float(f),
    // year, month, day, hour, minutes, seconds, micro seconds
    // mysql::Value::Date(u16, u8, u8, u8, u8, u8, u32)
    // mysql::Value::Time(bool, u32, u8, u8, u8, u32)
    _ => CasVal::Unknown,
  }
}
// impl From<mysql::Value> for CasVal {
//   fn from(my_val: mysql::Value) -> Self {
//     match my_val {
//       mysql::Value::NULL => CasVal::Null,
//       mysql::Value::Bytes(bytes) => CasVal::Str(String::from_utf8(bytes).unwrap()),
//       mysql::Value::Int(i) => CasVal::Int(i),
//       mysql::Value::UInt(u) => CasVal::Uint(u),
//       mysql::Value::Float(f) => CasVal::Float(f),
//       // year, month, day, hour, minutes, seconds, micro seconds
//       // mysql::Value::Date(u16, u8, u8, u8, u8, u8, u32)
//       // mysql::Value::Time(bool, u32, u8, u8, u8, u32)
//       _ => CasVal::Unknown,
//     }
//   }
// }
