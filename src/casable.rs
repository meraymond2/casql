use chrono::{DateTime, Local, Utc};
use postgres::types;
use postgres::types::{FromSql, Type};
use serde::ser::{Serialize, Serializer};
use serde_derive::Serialize;

#[derive(Clone, Debug, Serialize)]
pub enum CasVal {
    Str(String),
    UUID(String),
    Bool(bool),
    // UtcDate(DateTime<Utc>),
    // LocalDate(DateTime<Local>),
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

// impl Serialize for CasVal {
//   fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//   where
//     S: Serializer,
//   {
//     match self {
//       CasVal::Str(string) => serializer.serialize_str(string),
//       CasVal::UUID(string) => serializer.serialize_str(string),
//       CasVal::Null => serializer.serialize_unit(),
//       CasVal::Bool(b) => serializer.serialize_bool(*b),
//       CasVal::Int32(n) => serializer.serialize_i32(*n),
//       CasVal::Int64(n) => serializer.serialize_i64(*n),
//       CasVal::UInt32(n) => serializer.serialize_u32(*n),
//       CasVal::UInt64(n) => serializer.serialize_u64(*n),
//       CasVal::Float32(n) => serializer.serialize_f32(*n),
//       CasVal::Float64(n) => serializer.serialize_f64(*n),
//       CasVal::UtcDate(date) => serializer.serialize_str(&date.to_string()),
//       CasVal::LocalDate(date) => serializer.serialize_str(&date.to_string()),
//       CasVal::Json(json) => serializer.serialize_str(&json.to_string()),
//       CasVal::Unknown => serializer.serialize_str("???"),
//     }
//   }
// }

impl FromSql<'_> for CasVal {
    fn from_sql(ty: &Type, raw: &[u8]) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        let val = match ty {
            // &types::Type::UUID => {
            //     let x: uuid::Uuid = FromSql::from_sql(ty, raw)?;
            //     CasVal::UUID(x.to_string())
            // }
            &types::Type::TEXT => {
                let x: String = FromSql::from_sql(ty, raw)?;
                CasVal::Str(x)
            }
            &types::Type::VARCHAR => {
                let x: String = FromSql::from_sql(ty, raw)?;
                CasVal::Str(x)
            }
            // &types::Type::TIMESTAMP => {
            //     let x: DateTime<Utc> = FromSql::from_sql(ty, raw)?;
            //     CasVal::UtcDate(x)
            // }
            // &types::Type::TIMESTAMPTZ => {
            //     let x: DateTime<Local> = FromSql::from_sql(ty, raw)?;
            //     CasVal::LocalDate(x)
            // }
            &types::Type::CHAR => {
                let x: i8 = FromSql::from_sql(ty, raw)?;
                CasVal::Int32(x.into())
            }
            &types::Type::INT2 => {
                let x: i16 = FromSql::from_sql(ty, raw)?;
                CasVal::Int32(x.into())
            }
            &types::Type::INT4 => {
                let x: i32 = FromSql::from_sql(ty, raw)?;
                CasVal::Int32(x.into())
            }
            &types::Type::INT8 => {
                let x: i64 = FromSql::from_sql(ty, raw)?;
                CasVal::Int64(x)
            }
            &types::Type::FLOAT4 => {
                let val: f32 = FromSql::from_sql(ty, raw)?;
                CasVal::Float32(val.into())
            }
            &types::Type::FLOAT8 => {
                let val: f64 = FromSql::from_sql(ty, raw)?;
                CasVal::Float64(val)
            }
            &types::Type::BOOL => {
                let val: bool = FromSql::from_sql(ty, raw)?;
                CasVal::Bool(val)
            }
            &types::Type::JSON | &types::Type::JSONB => {
                let val: serde_json::Value = FromSql::from_sql(ty, raw)?;
                CasVal::Json(val)
            }
            _other => {
                // This gets me the vals, and I can get the oid as well. I think I would need
                // to get the matching value from another table.
                // match other.kind() {
                //   postgres::types::Kind::Enum(vals) => println!("Hey, an enum {:?}", vals),
                //   _ => println!("???, who knows"),
                // }
                eprintln!("Unrecognised type: {:?}", *ty);
                CasVal::Unknown
            }
        };
        Ok(val)
    }
    //
//   fn from_sql_null(_ty: &Type) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
//     Ok(CasVal::Null)
//   }
//
    fn accepts(_ty: &Type) -> bool {
        true
    }
}
