use chrono::{DateTime, NaiveDateTime, Local, Utc};
use mysql;
use mysql::consts::ColumnType;
use serde::ser::{Serialize, Serializer};
use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum CasVal {
  Str(String),
  UUID(String),
  Bool(bool),
  UtcDate(DateTime<Utc>),
  LocalDate(DateTime<Local>),
  Int32(i32),
  Int64(i64),
  UInt32(u32),
  UInt64(u64),
  Float32(f32),
  Float64(f64),
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
      CasVal::Int32(n) => serializer.serialize_i32(*n),
      CasVal::Int64(n) => serializer.serialize_i64(*n),
      CasVal::UInt32(n) => serializer.serialize_u32(*n),
      CasVal::UInt64(n) => serializer.serialize_u64(*n),
      CasVal::Float32(n) => serializer.serialize_f32(*n),
      CasVal::Float64(n) => serializer.serialize_f64(*n),
      CasVal::UtcDate(date) => serializer.serialize_str(&date.to_string()),
      CasVal::LocalDate(date) => serializer.serialize_str(&date.to_string()),
      CasVal::Json(json) => serializer.serialize_str(&json.to_string()),
      CasVal::Unknown => serializer.serialize_str("???"),
    }
  }
}

// I’m guessing that the specific mysql::Values are only populated when you
// specify a return type. When I instantiate them simply as Value, I just get
// Bytes, except for the NULLs.
pub fn from_mysql_value(my_val: mysql::Value, ty: mysql::consts::ColumnType) -> CasVal {
  match my_val {
    mysql::Value::NULL => CasVal::Null,
    mysql::Value::Int(_) => CasVal::Unknown,   // unused
    mysql::Value::UInt(_) => CasVal::Unknown,  // unused
    mysql::Value::Float(_) => CasVal::Unknown, // unused
    mysql::Value::Date(_, _, _, _, _, _, _) => CasVal::Unknown, // unused
    mysql::Value::Time(_, _, _, _, _, _) => CasVal::Unknown, // unused
    mysql::Value::Bytes(bytes) => {
      let s = String::from_utf8(bytes).unwrap();
      match ty {
        // Null — unused, goes to mysql::Value::Null instead
        // ColumnType::MYSQL_TYPE_NULL

        // Numerical Types
        //// Integers can be signed or unsigned
        ColumnType::MYSQL_TYPE_TINY => CasVal::Int32(s.parse().unwrap()),
        ColumnType::MYSQL_TYPE_SHORT => CasVal::Int32(s.parse().unwrap()),
        ColumnType::MYSQL_TYPE_INT24 => CasVal::Int32(s.parse().unwrap()),
        ColumnType::MYSQL_TYPE_LONG => match i32::from_str(&s) {
          Ok(i) => CasVal::Int32(i),
          Err(_) => CasVal::UInt32(s.parse().unwrap()),
        },
        ColumnType::MYSQL_TYPE_LONGLONG => match i64::from_str(&s) {
          Ok(i) => CasVal::Int64(i),
          Err(_) => CasVal::UInt64(s.parse().unwrap()),
        },

        //// Floating point numbers
        // ColumnType::MYSQL_TYPE_DECIMAL       // to do
        // ColumnType::MYSQL_TYPE_NEWDECIMAL    // to do
        ColumnType::MYSQL_TYPE_FLOAT => CasVal::Float32(s.parse().unwrap()),
        ColumnType::MYSQL_TYPE_DOUBLE => CasVal::Float64(s.parse().unwrap()),

        // String Types
        ColumnType::MYSQL_TYPE_VARCHAR => CasVal::Str(s),

        // Date Types
        ColumnType::MYSQL_TYPE_TIMESTAMP => {
          // TODO: Convert to UTC Datetime, or TZ, whichever this represents
          // the string looks like "2020-01-08 22:00:14"
          // TODO: How to do this when the format isn't known...hmmm
          // you can specify up to 6 microseconds
          // might need a loop
          // 2020-01-09 21:35:41
          // 2020-01-09 21:35:41.0000
          println!("{}", &s);
          println!("{:?}", &ty);

          // match NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S") {
          //   Ok(d) => println!("{:?}", d),
          //   Err(e) => println!("{:?}", e),
          // };
          // let date: DateTime<Utc> = DateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S").unwrap();
          CasVal::Unknown
        }
        // // ColumnType::MYSQL_TYPE_DATE
        // // ColumnType::MYSQL_TYPE_TIME
        // ColumnType::MYSQL_TYPE_DATETIME => {
        //   // TODO: Convert to UTC Datetime, or TZ, whichever this represents
        //   // the string looks like "2020-01-08 22:00:14"
        //   CasVal::Str(s)
        // }
        // ColumnType::MYSQL_TYPE_YEAR
        // ColumnType::MYSQL_TYPE_NEWDATE
        // ColumnType::MYSQL_TYPE_BIT
        // ColumnType::MYSQL_TYPE_TIMESTAMP2
        // ColumnType::MYSQL_TYPE_DATETIME2
        // ColumnType::MYSQL_TYPE_TIME2
        // ColumnType::MYSQL_TYPE_JSON
        // ColumnType::MYSQL_TYPE_ENUM
        // ColumnType::MYSQL_TYPE_SET
        // ColumnType::MYSQL_TYPE_TINY_BLOB
        // ColumnType::MYSQL_TYPE_MEDIUM_BLOB
        // ColumnType::MYSQL_TYPE_LONG_BLOB
        ColumnType::MYSQL_TYPE_BLOB => {
          // What's a blob???
          CasVal::Str(s)
        }
        ColumnType::MYSQL_TYPE_VAR_STRING => CasVal::Str(s),
        ColumnType::MYSQL_TYPE_STRING => CasVal::Str(s),
        // ColumnType::MYSQL_TYPE_GEOMETRY
        _ => {
          eprintln!("As yet unsupported MySQL type: {:?}", ty);
          CasVal::Str("Cascat!!!!".to_owned())
        }
      }
    }
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
