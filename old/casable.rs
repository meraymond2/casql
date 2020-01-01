use postgres::types;
use postgres::types::{FromSql, Type};
use serde::ser::{Serialize, Serializer};

use chrono::{DateTime, Utc, Local};
#[derive(Clone, Debug)]
pub enum CasableValue {
  CasString(String),
  CasUUID(String),
  CasBool(bool),
  CasUtcDate(DateTime<Utc>),
  CasLocalDate(DateTime<Local>),
  CasInt(i64),
  CasFloat(f64),
  CasJson(serde_json::Value),
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
      CasableValue::CasLocalDate(date) => serializer.serialize_str(&date.to_string()),
      CasableValue::CasJson(json) => serializer.serialize_str(&json.to_string()),
      CasableValue::CasUnknown => serializer.serialize_str("????"),
    }
  }
}

impl FromSql for CasableValue {
  fn from_sql(ty: &Type, raw: &[u8]) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
    let val = match ty {
      &types::UUID => {
        let x: uuid::Uuid = FromSql::from_sql(ty, raw)?;
        CasableValue::CasUUID(x.to_string())
      }
      &types::TEXT => {
        let x: String = FromSql::from_sql(ty, raw)?;
        CasableValue::CasString(x)
      }
      &types::VARCHAR => {
        let x: String = FromSql::from_sql(ty, raw)?;
        CasableValue::CasString(x)
      }
      &types::TIMESTAMP => {
        let x: DateTime<Utc> = FromSql::from_sql(ty, raw)?;
        CasableValue::CasUtcDate(x)
      }
      &types::TIMESTAMPTZ => {
        let x: DateTime<Local> = FromSql::from_sql(ty, raw)?;
        CasableValue::CasLocalDate(x)
      }
      &types::CHAR => {
        let x: i8 = FromSql::from_sql(ty, raw)?;
        CasableValue::CasInt(x.into())
      }
      &types::INT2 => {
        let x: i16 = FromSql::from_sql(ty, raw)?;
        CasableValue::CasInt(x.into())
      }
      &types::INT4 => {
        let x: i32 = FromSql::from_sql(ty, raw)?;
        CasableValue::CasInt(x.into())
      }
      &types::INT8 => {
        let x: i64 = FromSql::from_sql(ty, raw)?;
        CasableValue::CasInt(x)
      }
      &types::FLOAT4 => {
        let val: f32 = FromSql::from_sql(ty, raw)?;
        CasableValue::CasFloat(val.into())
      }
      &types::FLOAT8 => {
        let val: f64 = FromSql::from_sql(ty, raw)?;
        CasableValue::CasFloat(val)
      }
      &types::BOOL => {
        let val: bool = FromSql::from_sql(ty, raw)?;
        CasableValue::CasBool(val)
      }
      &types::JSON | &types::JSONB => {
        let val: serde_json::Value = FromSql::from_sql(ty, raw)?;
        CasableValue::CasJson(val)
      }
      _other => {
        // This gets me the vals, and I can get the oid as well. I think I would need
        // to get the matching value from another table.
        // match other.kind() {
        //   postgres::types::Kind::Enum(vals) => println!("Hey, an enum {:?}", vals),
        //   _ => println!("???, who knows"),
        // }
        eprintln!("Unrecognised type: {:?}", *ty);
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
