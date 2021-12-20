use casql::args;
use casql::cas_err::CasErr;
use casql::postgres;
use casql::postgres::connection::Conn;

// Requires local test database to be running.

/*
                                                 point
-------------------------------------------------------------------------------------------------------
 {"type":"Point","coordinates":[1.2,3.4]}
 {"type":"Point","coordinates":[1.2,3.4,5.6]}
 {"type":"Point","coordinates":[1.2,3.4]}
 {"type":"Point","coordinates":[1.2,3.4,5.6]}
 {"type":"Point","crs":{"type":"name","properties":{"name":"EPSG:32632"}},"coordinates":[1.2,3.4]}
 {"type":"Point","crs":{"type":"name","properties":{"name":"EPSG:32632"}},"coordinates":[1.2,3.4,5.6]}
 {"type":"Point","crs":{"type":"name","properties":{"name":"EPSG:32632"}},"coordinates":[1.2,3.4,5.6]}
 {"type":"Point","crs":{"type":"name","properties":{"name":"EPSG:32632"}},"coordinates":[1.2,3.4]}
 {"type":"Point","coordinates":[1.2,3.4,5.6]}
*/
#[test]
fn test_points() -> Result<(), CasErr> {
    let mut conn = connect()?;
    let mut out = Vec::new();
    conn.query("SELECT * FROM points".to_string(), vec![], &mut out)?;
    let expected = format!(
        "[{},{},{},{},{},{},{},{},{}]\n",
        r#"{"point":{"type":"Point","coordinates":[1.2,3.4]}}"#,
        r#"{"point":{"type":"Point","coordinates":[1.2,3.4,5.6]}}"#,
        r#"{"point":{"type":"Point","coordinates":[1.2,3.4,5.6]}}"#,
        r#"{"point":{"type":"Point","coordinates":[1.2,3.4,5.6,7.8]}}"#,
        r#"{"point":{"type":"Point","crs":{"type":"name","properties":{"name":"EPSG:32632"}},"coordinates":[1.2,3.4]}}"#,
        r#"{"point":{"type":"Point","crs":{"type":"name","properties":{"name":"EPSG:32632"}},"coordinates":[1.2,3.4,5.6]}}"#,
        r#"{"point":{"type":"Point","crs":{"type":"name","properties":{"name":"EPSG:32632"}},"coordinates":[1.2,3.4,5.6]}}"#,
        r#"{"point":{"type":"Point","crs":{"type":"name","properties":{"name":"EPSG:32632"}},"coordinates":[1.2,3.4,5.6]}}"#,
        r#"{"point":{"type":"Point","crs":{"type":"name","properties":{"name":"EPSG:4326"}},"coordinates":[1.2,3.4,5.6,7.8]}}"#,
    );
    assert_eq!(std::str::from_utf8(&out).unwrap(), expected);
    Ok(())
}

/*
                                      multipoint
--------------------------------------------------------------------------------------
 {"type":"MultiPoint","coordinates":[[13.595,56.429],[14.287,56.343],[14.15,56.731]]}
 */
#[test]
fn test_multipoints() -> Result<(), CasErr> {
    let mut conn = connect()?;
    let mut out = Vec::new();
    conn.query("SELECT * FROM multipoints".to_string(), vec![], &mut out)?;
    let expected = format!(
        "[{}]\n",
        r#"{"multipoint":{"type":"MultiPoint","coordinates":[[13.595,56.429],[14.287,56.343],[14.15,56.731]]}}"#,
    );
    assert_eq!(std::str::from_utf8(&out).unwrap(), expected);
    Ok(())
}

/*
                                           line
------------------------------------------------------------------------------------------
 {"type":"LineString","coordinates":[[1.2,3.4],[5.6,7.8],[9.1,11.12]]}
 {"type":"LineString","coordinates":[[1.2,3.4],[5.6,7.8],[9.1,11.12]]}
 {"type":"LineString","coordinates":[[1.2,3.4],[7.8,9.1],[13.14,15.16]]}
 {"type":"LineString","coordinates":[[1.2,3.4],[7.8,9.1],[13.14,15.16]]}
 {"type":"LineString","coordinates":[[1.2,3.4,5.6],[7.8,9.1,11.12],[13.14,15.16,17.18]]}
 {"type":"LineString","coordinates":[[1.2,3.4,5.6],[7.8,9.1,11.12],[13.14,15.16,17.18]]}
 {"type":"LineString","coordinates":[[1.2,3.4,5.6],[9.1,11.12,13.14],[17.18,19.2,21.22]]}
*/
#[test]
fn test_lines() -> Result<(), CasErr> {
    let mut conn = connect()?;
    let mut out = Vec::new();
    conn.query("SELECT * FROM lines".to_string(), vec![], &mut out)?;
    let expected = format!(
        "[{},{},{},{},{},{},{}]\n",
        r#"{"line":{"type":"LineString","coordinates":[[1.2,3.4],[5.6,7.8],[9.1,11.12]]}}"#,
        r#"{"line":{"type":"LineString","crs":{"type":"name","properties":{"name":"EPSG:4326"}},"coordinates":[[1.2,3.4],[5.6,7.8],[9.1,11.12]]}}"#,
        r#"{"line":{"type":"LineString","coordinates":[[1.2,3.4,5.6],[7.8,9.1,11.12],[13.14,15.16,17.18]]}}"#,
        r#"{"line":{"type":"LineString","crs":{"type":"name","properties":{"name":"EPSG:4326"}},"coordinates":[[1.2,3.4,5.6],[7.8,9.1,11.12],[13.14,15.16,17.18]]}}"#,
        r#"{"line":{"type":"LineString","coordinates":[[1.2,3.4,5.6],[7.8,9.1,11.12],[13.14,15.16,17.18]]}}"#,
        r#"{"line":{"type":"LineString","crs":{"type":"name","properties":{"name":"EPSG:4326"}},"coordinates":[[1.2,3.4,5.6],[7.8,9.1,11.12],[13.14,15.16,17.18]]}}"#,
        r#"{"line":{"type":"LineString","coordinates":[[1.2,3.4,5.6,7.8],[9.1,11.12,13.14,15.16],[17.18,19.2,21.22,23.24]]}}"#,
    );
    assert_eq!(std::str::from_utf8(&out).unwrap(), expected);
    Ok(())
}

/*
                                                                   multiline
------------------------------------------------------------------------------------------------------------------------------------------------
 {"type":"MultiLineString","coordinates":[[[14.172,56.829],[14.243,57.087],[14.889,57.113]],[[14.331,56.354],[15.067,56.479],[15.185,56.745]]]}
 */
#[test]
fn test_multilines() -> Result<(), CasErr> {
    let mut conn = connect()?;
    let mut out = Vec::new();
    conn.query("SELECT * FROM multilines".to_string(), vec![], &mut out)?;
    let expected = format!(
        "[{}]\n",
        r#"{"multiline":{"type":"MultiLineString","coordinates":[[[14.172,56.829],[14.243,57.087],[14.889,57.113]],[[14.331,56.354],[15.067,56.479],[15.185,56.745]]]}}"#,
    );
    assert_eq!(std::str::from_utf8(&out).unwrap(), expected);
    Ok(())
}

/*
                                                                           poly
-----------------------------------------------------------------------------------------------------------------------------------------------------------
 {"type":"Polygon","coordinates":[[[10.60798645,59.805987373],[10.70514679,59.799598075],[10.654335022,59.870502384],[10.60798645,59.805987373]]]}
 {"type":"Polygon","coordinates":[[[10.60798645,59.805987373],[10.70514679,59.799598075],[10.654335022,59.870502384],[10.60798645,59.805987373]]]}
 {"type":"Polygon","coordinates":[[[10.60798645,59.805987373],[10.70514679,59.799598075],[10.654335022,59.870502384],[10.60798645,59.805987373]]]}
 {"type":"Polygon","coordinates":[[[10.60798645,59.805987373],[10.70514679,59.799598075],[10.654335022,59.870502384],[10.60798645,59.805987373]]]}
 {"type":"Polygon","coordinates":[[[10.60798645,59.805987373,1],[10.70514679,59.799598075,2],[10.654335022,59.870502384,3],[10.60798645,59.805987373,4]]]}
 {"type":"Polygon","coordinates":[[[10.60798645,59.805987373,1],[10.70514679,59.799598075,2],[10.654335022,59.870502384,3],[10.60798645,59.805987373,4]]]}
 {"type":"Polygon","coordinates":[[[10.60798645,59.805987373,5],[10.70514679,59.799598075,5],[10.654335022,59.870502384,5],[10.60798645,59.805987373,5]]]}
 {"type":"Polygon","coordinates":[[[10.60798645,59.805987373,5],[10.70514679,59.799598075,5],[10.654335022,59.870502384,5],[10.60798645,59.805987373,5]]]}
 */

#[test]
fn test_polys() -> Result<(), CasErr> {
    let mut conn = connect()?;
    let mut out = Vec::new();
    conn.query("SELECT * FROM polys".to_string(), vec![], &mut out)?;
    let expected = format!(
        "[{},{},{},{},{},{},{},{}]\n",
        r#"{"poly":{"type":"Polygon","coordinates":[[[10.607986450195313,59.80598737346893],[10.705146789550781,59.799598075478414],[10.654335021972656,59.87050238394518],[10.607986450195313,59.80598737346893]]]}}"#,
        r#"{"poly":{"type":"Polygon","crs":{"type":"name","properties":{"name":"EPSG:4326"}},"coordinates":[[[10.607986450195313,59.80598737346893],[10.705146789550781,59.799598075478414],[10.654335021972656,59.87050238394518],[10.607986450195313,59.80598737346893]]]}}"#,
        r#"{"poly":{"type":"Polygon","coordinates":[[[10.607986450195313,59.80598737346893,1],[10.705146789550781,59.799598075478414,2],[10.654335021972656,59.87050238394518,3],[10.607986450195313,59.80598737346893,4]]]}}"#,
        r#"{"poly":{"type":"Polygon","crs":{"type":"name","properties":{"name":"EPSG:4326"}},"coordinates":[[[10.607986450195313,59.80598737346893,1],[10.705146789550781,59.799598075478414,2],[10.654335021972656,59.87050238394518,3],[10.607986450195313,59.80598737346893,4]]]}}"#,
        r#"{"poly":{"type":"Polygon","coordinates":[[[10.607986450195313,59.80598737346893,1],[10.705146789550781,59.799598075478414,2],[10.654335021972656,59.87050238394518,3],[10.607986450195313,59.80598737346893,4]]]}}"#,
        r#"{"poly":{"type":"Polygon","crs":{"type":"name","properties":{"name":"EPSG:4326"}},"coordinates":[[[10.607986450195313,59.80598737346893,1],[10.705146789550781,59.799598075478414,2],[10.654335021972656,59.87050238394518,3],[10.607986450195313,59.80598737346893,4]]]}}"#,
        r#"{"poly":{"type":"Polygon","coordinates":[[[10.607986450195313,59.80598737346893,5,1],[10.705146789550781,59.799598075478414,5,2],[10.654335021972656,59.87050238394518,5,3],[10.607986450195313,59.80598737346893,5,4]]]}}"#,
        r#"{"poly":{"type":"Polygon","crs":{"type":"name","properties":{"name":"EPSG:4326"}},"coordinates":[[[10.607986450195313,59.80598737346893,5,1],[10.705146789550781,59.799598075478414,5,2],[10.654335021972656,59.87050238394518,5,3],[10.607986450195313,59.80598737346893,5,4]]]}}"#,
    );
    assert_eq!(std::str::from_utf8(&out).unwrap(), expected);
    Ok(())
}

/*
                                                                            multipoly
------------------------------------------------------------------------------------------------------------------------------------------------------------------
 {"type":"MultiPolygon","coordinates":[[[[40,40],[20,45],[45,30],[40,40]]],[[[20,35],[10,30],[10,10],[30,5],[45,20],[20,35]],[[30,20],[20,15],[20,25],[30,20]]]]}
*/
#[test]
fn test_multipolys() -> Result<(), CasErr> {
    let mut conn = connect()?;
    let mut out = Vec::new();
    conn.query("SELECT * FROM multipolys".to_string(), vec![], &mut out)?;
    let expected = format!(
        "[{}]\n",
        r#"{"multipoly":{"type":"MultiPolygon","coordinates":[[[[40,40],[20,45],[45,30],[40,40]]],[[[20,35],[10,30],[10,10],[30,5],[45,20],[20,35]],[[30,20],[20,15],[20,25],[30,20]]]]}}"#,
    );
    assert_eq!(std::str::from_utf8(&out).unwrap(), expected);
    Ok(())
}
/*
                                                                                    coll
------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
 {"type":"GeometryCollection","geometries":[{"type":"Point","coordinates":[40,10]},{"type":"LineString","coordinates":[[10,10],[20,20],[10,40]]},{"type":"Polygon","coordinates":[[[40,40],[20,45],[45,30],[40,40]]]}]}
*/
#[test]
fn test_geometry_collections() -> Result<(), CasErr> {
    let mut conn = connect()?;
    let mut out = Vec::new();
    conn.query("SELECT * FROM geo_coll".to_string(), vec![], &mut out)?;
    let expected = format!(
        "[{}]\n",
        r#"{"coll":{"type":"GeometryCollection","geometries":[{"type":"Point","coordinates":[40,10]},{"type":"LineString","coordinates":[[10,10],[20,20],[10,40]]},{"type":"Polygon","coordinates":[[[40,40],[20,45],[45,30],[40,40]]]}]}}"#,
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
        postgis: true,
    };
    postgres::connection::Conn::connect(params)
}
