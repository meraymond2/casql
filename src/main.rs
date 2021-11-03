mod binary_reader;
mod cas_err;
mod postgres;
use crate::cas_err::CasErr;
use postgres::connection::{Conn, ConnectionParams};
mod cas_val;
mod json;

fn main() {
    match exec_query() {
        Ok(_) => std::process::exit(0),
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}

fn exec_query() -> Result<(), CasErr> {
    let params = ConnectionParams {
        user: "michael".to_owned(),
        database: Some("dbname".to_owned()),
        port: Some(5432),
        host: "localhost".to_owned(),
        password: Some("cascat".to_owned()),
        postgis: true,
    };
    let mut conn = Conn::connect(params)?;
    conn.query(String::from("SELECT typname FROM pg_type"), vec![], json::write_json)
    // conn.query(String::from("SELECT column_name AS field, data_type AS type, column_default AS default, is_nullable = 'YES' AS nullable FROM INFORMATION_SCHEMA.COLUMNS WHERE table_name = 'points';"), vec![], json::write_json)
    // conn.query(
    //     String::from("SELECT * FROM points"),
    //     vec![],
    //     json::write_json,
    // )
}
