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
        "[{}]\n",
        r#"{"bool":true,"int2":12345,"int4":12345678,"int8":123456790123}"#
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
        "[{},{},{},{}]\n",
        r#"{"float4":3.1415927,"float8":3.141592653589793}"#,
        r#"{"float4":"NaN","float8":"NaN"}"#,
        r#"{"float4":"Infinity","float8":"Infinity"}"#,
        r#"{"float4":"-Infinity","float8":"-Infinity"}"#,
    );
    assert_eq!(out, expected.as_bytes());
    Ok(())
}

/*
    numeric    | zero_scale | fixed_scale
---------------+------------+-------------
       1234567 |      12346 | 12345.54321
             1 |          2 |     3.00000
  0.0000000002 |          0 |     0.00003
 -0.0000000002 |          0 |    -0.00003
  100000000000 | 1000000000 | 10000.00000
               |            |
           NaN |        NaN |         NaN
 */
#[test]
fn test_numerics() -> Result<(), CasErr> {
    let mut conn = connect()?;
    let mut out = Vec::new();
    conn.query("SELECT * FROM numerics".to_string(), vec![], &mut out)?;
    let expected = format!(
        "[{},{},{},{},{},{},{}]\n",
        r#"{"numeric":1234567,"zero_scale":12346,"fixed_scale":12345.54321}"#,
        r#"{"numeric":1,"zero_scale":2,"fixed_scale":3.00000}"#,
        r#"{"numeric":0.0000000002,"zero_scale":0,"fixed_scale":0.00003}"#,
        r#"{"numeric":-0.0000000002,"zero_scale":0,"fixed_scale":-0.00003}"#,
        r#"{"numeric":100000000000,"zero_scale":1000000000,"fixed_scale":10000.00000}"#,
        r#"{"numeric":null,"zero_scale":null,"fixed_scale":null}"#,
        r#"{"numeric":"NaN","zero_scale":"NaN","fixed_scale":"NaN"}"#,
    );
    assert_eq!(out, expected.as_bytes());
    Ok(())
}

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
        "[{}]\n",
        r#"{"char":"O","fixed_char":"wee","name":"sleekrit","text":"cowran","varchar":"timrous","bounded_varchar":"\"beastie\""}"#
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
