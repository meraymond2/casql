use casql::args;
use casql::cas_err::CasErr;
use casql::postgres;
use casql::postgres::connection::Conn;

// Requires local test database to be running.

/*
 bool | int2  |   int4   |     int8     | float4  |      float8       |           numeric           |    zero_scale    |        fixed_scale
------+-------+----------+--------------+---------+-------------------+-----------------------------+------------------+----------------------------
 t    | 12345 | 12345678 | 123456790123 | 3.14159 | 3.141592653589793 | 3.1415926535897932384626433 | 9007199254740992 | 3.141592653589793238462643
*/
#[test]
fn test_numbers() -> Result<(), CasErr> {
    let mut conn = connect()?;
    let mut out = Vec::new();
    conn.query("SELECT * FROM numbers".to_string(), vec![], &mut out)?;
    let expected = format!(
        "{}\n",
        r#"[{"bool":true,"int2":12345,"int4":12345678,"int8":123456790123,"float4":"3.14159","float8":"3.141592653589793","numeric":"3.1415926535897932384626433","zero_scale":9007199254740992,"fixed_scale":3.141592653589793238462643}]"#
    );
    assert_eq!(out, expected.as_bytes());
    Ok(())
} // TODO: add a test for NaN and Infinitys

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
