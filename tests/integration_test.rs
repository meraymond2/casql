use casql::args;
use casql::cas_err::CasErr;
use casql::postgres;
use casql::postgres::connection::Conn;

// Requires local test database to be running.

#[test]
fn test_query() -> Result<(), CasErr> {
    let mut conn = connect()?;
    let mut out = Vec::new();
    conn.query("SELECT * FROM types_0_99".to_string(), vec![], &mut out)?;
    let expected = format!(
        "{}\n",
        r#"[{"bool":true,"bytea":[0,1,10,255],"char":"A","name":"name is michael","int8":1234567890,"int2":12345,"int4":1234567890,"regproc":77,"text":"I’m a Postgres text value, how do you like me so far?","oid":77,"tid":[9,8],"xid":42,"cid":34}]"#
    );
    assert_eq!(out, expected.as_bytes());
    Ok(())
}

fn connect() -> Result<Conn, CasErr> {
    let params = args::ConnectionParams {
        host: "localhost".to_string(),
        user: "root".to_string(),
        password: Some("cascat".to_string()),
        database: Some("dbname".to_string()),
        port: Some(5432),
        postgis: false,
    };
    postgres::connection::Conn::connect(params)
}
