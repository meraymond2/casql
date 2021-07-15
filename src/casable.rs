use chrono::{DateTime, Local, Utc};
use postgres::types;
use postgres::types::{FromSql, Type};
use serde_derive::Serialize;

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum CasVal {
    Str(String),
    UUID(String),
    Bool(bool),
    Int32(i32),
    Int64(i64),
    Float32(f32),
    Float64(f64),
    Json(serde_json::Value),
    Null,
    Unknown,
}

impl FromSql<'_> for CasVal {
    fn from_sql(ty: &Type, raw: &[u8]) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        let val = match ty {
            &types::Type::UUID => {
                let x: uuid::Uuid = FromSql::from_sql(ty, raw)?;
                CasVal::UUID(x.to_string())
            }
            &types::Type::TEXT => {
                let x: String = FromSql::from_sql(ty, raw)?;
                CasVal::Str(x)
            }
            &types::Type::VARCHAR => {
                let x: String = FromSql::from_sql(ty, raw)?;
                CasVal::Str(x)
            }
            &types::Type::TIMESTAMP => {
                // todo: make not utc but non-timestamped datetime string, i.e. lose the Z
                let x: DateTime<Utc> = FromSql::from_sql(ty, raw)?;
                CasVal::Str(x.to_string())
            }
            &types::Type::TIMESTAMPTZ => {
                let x: DateTime<Local> = FromSql::from_sql(ty, raw)?;
                CasVal::Str(x.to_string())
            }
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
                // I might need to do that for the PostGIS types, there are types for the individual
                // geometry types, but they don’t seem to get picked up, even when the column is a
                // specific one, like geometry(Point,4326).
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

    fn from_sql_null(_ty: &Type) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        Ok(CasVal::Null)
    }

    fn accepts(_ty: &Type) -> bool {
        true
    }
}
