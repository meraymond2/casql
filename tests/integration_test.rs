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
    assert_eq!(std::str::from_utf8(&out).unwrap(), expected);
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
    assert_eq!(std::str::from_utf8(&out).unwrap(), expected);
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
    assert_eq!(std::str::from_utf8(&out).unwrap(), expected);
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
    assert_eq!(std::str::from_utf8(&out).unwrap(), expected);
    Ok(())
}

/*
                            bytea                             | bit |  octet   | varbit |                          bounded_varbit
--------------------------------------------------------------+-----+----------+--------+------------------------------------------------------------------
 \x5468657265e2809973206e6f2073756368207468696e67206173203221 | 1   | 00001010 | 10101  | 0000000000000000000000000000000000000000000000000000000000001000
*/
#[test]
fn test_binaries() -> Result<(), CasErr> {
    let mut conn = connect()?;
    let mut out = Vec::new();
    conn.query("SELECT * FROM binaries".to_string(), vec![], &mut out)?;
    let expected = format!(
        "[{}]\n",
        r#"{"bytea":[84,104,101,114,101,226,128,153,115,32,110,111,32,115,117,99,104,32,116,104,105,110,103,32,97,115,32,50,33],"bit":"1","octet":"00001010","varbit":"10101","bounded_varbit":"0000000000000000000000000000000000000000000000000000000000001000"}"#
    );
    assert_eq!(std::str::from_utf8(&out).unwrap(), expected);
    Ok(())
}

/*
     date      |      time       |          timestamp           |           timestamptz           |                interval                |      timetz
---------------+-----------------+------------------------------+---------------------------------+----------------------------------------+-------------------
 4713-01-01 BC | 04:05:06.789    | 4713-01-01 04:05:06.789 BC   | 4713-01-01 04:05:06.789+00 BC   | 00:00:00                               | 04:05:06.789-08
 0002-12-31    | 00:00:00        | 0002-12-31 00:00:00          | 0002-12-30 23:00:00+00          | -1 years -2 mons -3 days -04:05:06     | 04:05:06-08
 2200-01-01    | 16:05:00        | 2200-01-01 16:05:00          | 2200-01-02 00:05:00+00          | -1 years -1 mons +12 days -00:12:47.88 | 04:05:06+03:02:01
 5874897-12-31 | 23:59:59.999999 | 294276-12-31 23:59:59.999999 | 294276-12-31 23:59:59.999999+00 | 00:00:00.000015                        | 04:05:06.789+00
*/
#[test]
fn test_dates_and_times() -> Result<(), CasErr> {
    let mut conn = connect()?;
    let mut out = Vec::new();
    conn.query(
        "SELECT * FROM dates_and_times".to_string(),
        vec![],
        &mut out,
    )?;
    let expected = format!(
        "[{},{},{},{}]\n",
        r#"{"date":"-4712-01-01","time":"04:05:06.789","timestamp":"-4712-01-01T04:05:06.789Z","timestamptz":"-4712-01-01T04:05:06.789Z","interval":"P0D","timetz":"04:05:06.789-08:00"}"#,
        r#"{"date":"0002-12-31","time":"00:00:00","timestamp":"0002-12-31T00:00:00Z","timestamptz":"0002-12-30T23:00:00Z","interval":"P-1Y-2M-3DT-4H-5M-6S","timetz":"04:05:06-08:00"}"#,
        r#"{"date":"2200-01-01","time":"16:05:00","timestamp":"2200-01-01T16:05:00Z","timestamptz":"2200-01-02T00:05:00Z","interval":"P-1Y-1M12DT-12M-47.88S","timetz":"04:05:06+03:02:01"}"#,
        r#"{"date":"5874897-12-31","time":"23:59:59.999999","timestamp":"294276-12-31T23:59:59.999999Z","timestamptz":"294276-12-31T23:59:59.999999Z","interval":"PT0.000015S","timetz":"04:05:06.789+00:00"}"#,
    );
    assert_eq!(std::str::from_utf8(&out).unwrap(), expected);
    Ok(())
}

/*
       json       |     jsonb      | jsonpath |       xml       |                 uuid
------------------+----------------+----------+-----------------+--------------------------------------
 { "cas": "cat" } | {"cas": "cat"} | $."cas"  | <div>html</div> | 27e31d5b-b544-44e0-83c1-379519b8a115
*/
#[test]
fn test_structured_data() -> Result<(), CasErr> {
    let mut conn = connect()?;
    let mut out = Vec::new();
    conn.query(
        "SELECT * FROM structured_data".to_string(),
        vec![],
        &mut out,
    )?;
    let expected = format!(
        "[{}]\n",
        r#"{"json":{"cas":"cat"},"jsonb":{"cas":"cat"},"jsonpath":"$.\"cas\"","xml":"<div>html</div>","uuid":"27e31d5b-b544-44e0-83c1-379519b8a115"}"#,
    );
    assert_eq!(std::str::from_utf8(&out).unwrap(), expected);
    Ok(())
}

/*
 point |     lseg      |        path         |     box     |          polygon          |  line   |   circle
-------+---------------+---------------------+-------------+---------------------------+---------+------------
 (2,4) | [(0,0),(2,4)] | ((0,0),(1,2),(2,4)) | (2,2),(0,0) | ((0,0),(2,2),(2,4),(0,0)) | {2,3,4} | <(0,0),10>
*/
#[test]
fn test_shapes() -> Result<(), CasErr> {
    let mut conn = connect()?;
    let mut out = Vec::new();
    conn.query("SELECT * FROM shapes".to_string(), vec![], &mut out)?;
    let expected = format!(
        "[{}]\n",
        r#"{"point":[2,4],"lseg":[[0,0],[2,4]],"path":[[0,0],[1,2],[2,4]],"box":[[2,2],[0,0]],"polygon":[[0,0],[2,2],[2,4],[0,0]],"line":"2x + 3y + 4 = 0","circle":[[0,0],10]}"#,
    );
    assert_eq!(std::str::from_utf8(&out).unwrap(), expected);
    Ok(())
}

/*
            cidr             |        macaddr8         |      macaddr      |            inet
-----------------------------+-------------------------+-------------------+-----------------------------
 192.168.100.128/25          | 08:00:2b:01:02:03:04:05 | 08:00:2b:01:02:03 | 127.0.0.1
 2001:db8::8a2e:370:7334/128 |                         |                   | 2001:db8::8a2e:370:7334/120
*/
#[test]
fn test_net() -> Result<(), CasErr> {
    let mut conn = connect()?;
    let mut out = Vec::new();
    conn.query("SELECT * FROM networking".to_string(), vec![], &mut out)?;
    let expected = format!(
        "[{},{}]\n",
        r#"{"cidr":"192.168.100.128/25","macaddr8":"08-00-2b-01-02-03-04-05","macaddr":"08-00-2b-01-02-03","inet":"127.0.0.1"}"#,
        r#"{"cidr":"2001:db8:0:0:0:8a2e:370:7334/128","macaddr8":null,"macaddr":null,"inet":"2001:db8:0:0:0:8a2e:370:7334/120"}"#,
    );
    assert_eq!(std::str::from_utf8(&out).unwrap(), expected);
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
