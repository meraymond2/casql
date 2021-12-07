use casql::args;
use casql::cas_err::CasErr;
use casql::postgres;
use casql::postgres::connection::Conn;

// Requires local test database to be running.

/*
 bool | int2  |   int4   |     int8     | float4  |      float8       |           numeric
------+-------+----------+--------------+---------+-------------------+-----------------------------
 t    | 12345 | 12345678 | 123456790123 | 3.14159 | 3.141592653589793 | 3.1415926535897932384626433
*/
#[test]
fn test_numbers() -> Result<(), CasErr> {
    let mut conn = connect()?;
    let mut out = Vec::new();
    conn.query("SELECT * FROM numbers".to_string(), vec![], &mut out)?;
    let expected = format!(
        "{}\n",
        r#"[{"bool":true,"int2":12345,"int4":12345678,"int8":123456790123,"float4":"3.14159","float8":"3.141592653589793","numeric":"3.1415926535897932384626433"}]"#
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
