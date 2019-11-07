extern crate chrono;
extern crate postgres;

use chrono::{DateTime, Utc};
use postgres::types::{FromSql, Type, BOOL, NUMERIC, TEXT, TIMESTAMP, UUID};
use postgres::{Connection, TlsMode};

#[derive(Debug)]
enum CasableValue {
    CasUUID(String),
    CasString(String),
    CasBool(bool),
    CasUtcDate(DateTime<Utc>),
    CasNull,
    CasUnknown,
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

fn main() {
    let conn = Connection::connect("postgres://root@localhost:5432/api-db", TlsMode::None).unwrap();
    let res = conn
        .query(
            "SELECT * FROM cats WHERE id = '73156aaa-31a8-4aca-a44c-a755e49b820f'",
            &[],
        )
        .unwrap();

    for row in res.into_iter() {
        let count = row.len();
        for i in 0..count {
            let n: CasableValue = row.get(i);
            println!("{:?}", n);
        }
    }
}
