use casql::args;
use casql::cas_err::CasErr;
use casql::postgres;
use casql::postgres::connection::Conn;

// Requires local test database to be running.

/*
 bool | int2  |   int4   |     int8
------+-------+----------+--------------
 t    | 12345 | 12345678 | 123456790123
*/
#[test]
fn test_integers() -> Result<(), CasErr> {
    let mut conn = connect()?;
    let mut out = Vec::new();
    conn.query("SELECT * FROM integers".to_string(), vec![], &mut out)?;
    let expected = format!(
        "{}\n",
        r#"[{"bool":true,"int2":12345,"int4":12345678,"int8":123456790123}]"#
    );
    assert_eq!(out, expected.as_bytes());
    Ok(())
}

/*
  float4   |      float8
-----------+-------------------
 3.1415927 | 3.141592653589793
       NaN |               NaN
  Infinity |          Infinity
 -Infinity |         -Infinity
*/
#[test]
fn test_floats() -> Result<(), CasErr> {
    let mut conn = connect()?;
    let mut out = Vec::new();
    conn.query("SELECT * FROM floats".to_string(), vec![], &mut out)?;
    let expected = format!(
        "{}\n",
        r#"[{"float4":3.1415927,"float8":3.141592653589793},{"float4":"NaN","float8":"NaN"},{"float4":"Infinity","float8":"Infinity"},{"float4":"-Infinity","float8":"-Infinity"}]"#
    );
    assert_eq!(out, expected.as_bytes());
    Ok(())
}

// TODO: add numerics test

/*
 char | fixed_char |   name   |  text  | varchar | bounded_varchar
------+------------+----------+--------+---------+-----------------
 O    | wee        | sleekrit | cowran | timrous | "beastie"
*/
#[test]
fn test_texts() -> Result<(), CasErr> {
    let mut conn = connect()?;
    let mut out = Vec::new();
    conn.query("SELECT * FROM texts".to_string(), vec![], &mut out)?;
    let expected = format!(
        "{}\n",
        r#"[{"char":"O","fixed_char":"wee","name":"sleekrit","text":"cowran","varchar":"timrous","bounded_varchar":"\"beastie\""}]"#
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
