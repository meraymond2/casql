use postgres::types::{FromSql, Type, BOOL, NUMERIC, TEXT, TIMESTAMP, UUID};
use serde::ser::{Serialize, SerializeStruct, Serializer};

use chrono::{DateTime, Utc};
#[derive(Clone, Debug)]
pub enum CasableValue {
  CasString(String),
  CasUUID(String),
  CasBool(bool),
  CasUtcDate(DateTime<Utc>),
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
      CasableValue::CasUtcDate(date) => serializer.serialize_str(&date.to_string()),
      CasableValue::CasUnknown => serializer.serialize_str("????"),
    }
  }
}

impl FromSql for CasableValue {
  fn from_sql(ty: &Type, raw: &[u8]) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
    let val = match *ty {
      UUID => {
        let x: uuid::Uuid = FromSql::from_sql(ty, raw)?;
        CasableValue::CasUUID(x.to_string())
      }
      TEXT => {
        let x: String = FromSql::from_sql(ty, raw)?;
        CasableValue::CasString(x)
      }
      TIMESTAMP => {
        let x: DateTime<Utc> = FromSql::from_sql(ty, raw)?;
        CasableValue::CasUtcDate(x)
      }
      // VARCHAR | TEXT | BPCHAR | NAME | UNKNOWN=>{
      //     let x:Option<String> = FromSql::from_sql(ty, raw)?;
      //     Scalar::from(x)
      // }
      // INT2 | INT4=>{
      //     let x:i32 = FromSql::from_sql(ty, raw)?;
      //     Scalar::I32(x)
      // }
      // INT8 =>{
      //     let x:i64 = FromSql::from_sql(ty, raw)?;
      //     Scalar::I64(x)
      // }
      NUMERIC => {
        println!("{:?}", raw);
        CasableValue::CasNull
      }
      BOOL => {
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
