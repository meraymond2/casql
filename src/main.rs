mod binary_reader;
mod cas_err;
mod postgres;
use crate::cas_err::CasErr;
use postgres::connection::{Conn, ConnectionParams};

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
        postgis: false,
    };
    let mut conn = Conn::connect(params)?;
    // conn.query(String::from("SELECT * FROM pg_type"), vec![])
    // conn.query(String::from("SELECT * FROM points"), vec![])
    Ok(())
}
