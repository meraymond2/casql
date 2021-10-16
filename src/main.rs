mod postgres;

use postgres::conn::{Conn, ConnectionParams};

fn main() -> std::io::Result<()> {
    let params = ConnectionParams {
        user: "michael".to_owned(),
        database: Some("dbname".to_owned()),
        port: Some(5432),
        host: "localhost".to_owned(),
        password: Some("cascat".to_owned()),
    };
    let mut conn = Conn::connect(params);
    if let Ok(mut c) = conn {
        c.query(String::from("SELECT * FROM pg_type LIMIT 3"), vec![]);
    }
    Ok(())
}
