use serde::Serialize;
// This isn’t a complete implementation of the GeoJSON spec, but rather an imitation of the output
// of ST_AsGeoJSON. Unlike ST_AsGeoJSON I always include the SRID if it’s set, whereas PostGIS
// only includes it if it’s set and not the default 4326.

// TODO: coordinates (in a generic way?)
// TODO: other types

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Position {
    XY(f64, f64),
    XYZ(f64, f64, f64),
    XYM(f64, f64, f64),
    XYZM(f64, f64, f64, f64),
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Coords {
    Point(Position),
    LineString(Vec<Position>),
}

// Not worried about enforcing valid GeoJSON with types, just abstracting over the different geometries.
#[derive(Debug, Serialize)]
pub struct GeoJSON {
    #[serde(rename = "type")]
    pub tag: GeoJSONType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crs: Option<CRS>,
    pub coordinates: Coords,
}

#[derive(Debug, Serialize)]
pub struct CRS {
    #[serde(rename = "type")]
    tag: &'static str, // always just "name"
    properties: Properties,
}

impl CRS {
    pub fn from(srid: Option<i32>) -> Option<CRS> {
        srid.map(|srid| CRS {
            tag: "name",
            properties: Properties {
                name: format!("EPSG:{}", srid),
            },
        })
    }
}

#[derive(Debug, Serialize)]
pub struct Properties {
    name: String,
}
#[derive(Debug, Serialize)]
pub enum GeoJSONType {
    Point,
}
