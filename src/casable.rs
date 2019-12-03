use postgres::types;
use postgres::types::{FromSql, Type};
use serde::ser::{Serialize, Serializer};

use chrono::{DateTime, Utc};
#[derive(Clone, Debug)]
pub enum CasableValue {
  CasString(String),
  CasUUID(String),
  CasBool(bool),
  CasUtcDate(DateTime<Utc>),
  CasInt(i64),
  CasFloat(f64),
  CasNull,
  CasUnknown,
}

impl Serialize for CasableValue {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    match self {
      CasableValue::CasString(string) => serializer.serialize_str(string),
      CasableValue::CasUUID(string) => serializer.serialize_str(string),
      CasableValue::CasNull => serializer.serialize_unit(),
      CasableValue::CasBool(b) => serializer.serialize_bool(*b),
      CasableValue::CasInt(n) => serializer.serialize_i64(*n),
      CasableValue::CasFloat(n) => serializer.serialize_f64(*n),
      CasableValue::CasUtcDate(date) => serializer.serialize_str(&date.to_string()),
      CasableValue::CasUnknown => serializer.serialize_str("????"),
    }
  }
}

impl FromSql for CasableValue {
  fn from_sql(ty: &Type, raw: &[u8]) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
    let val = match *ty {
      types::UUID => {
        let x: uuid::Uuid = FromSql::from_sql(ty, raw)?;
        CasableValue::CasUUID(x.to_string())
      }
      types::TEXT => {
        let x: String = FromSql::from_sql(ty, raw)?;
        CasableValue::CasString(x)
      }
      types::TIMESTAMP => {
        let x: DateTime<Utc> = FromSql::from_sql(ty, raw)?;
        CasableValue::CasUtcDate(x)
      }
      types::CHAR => {
        let x: i8 = FromSql::from_sql(ty, raw)?;
        CasableValue::CasInt(x.into())
      }
      types::INT2 => {
        let x: i16 = FromSql::from_sql(ty, raw)?;
        CasableValue::CasInt(x.into())
      }
      types::INT4 => {
        let x: i32 = FromSql::from_sql(ty, raw)?;
        CasableValue::CasInt(x.into())
      }
      types::INT8 => {
        let x: i64 = FromSql::from_sql(ty, raw)?;
        CasableValue::CasInt(x)
      }
      types::FLOAT4 => {
        let val: f32 = FromSql::from_sql(ty, raw)?;
        CasableValue::CasFloat(val.into())
      }
      types::FLOAT8 => {
        let val: f64 = FromSql::from_sql(ty, raw)?;
        CasableValue::CasFloat(val)
      }
      types::BOOL => {
        let val: bool = FromSql::from_sql(ty, raw)?;
        CasableValue::CasBool(val)
      }
      _ => {
        println!("Unrecognised type: {:?}", *ty);
        CasableValue::CasUnknown
      }
    };
    Ok(val)
  }

  fn from_sql_null(_ty: &Type) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
    Ok(CasableValue::CasNull)
  }

  fn accepts(_ty: &Type) -> bool {
    true
  }
}
