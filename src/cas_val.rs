use crate::postgres::postgis::geojson::GeoJSON;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum CasVal {
    Bool(bool),
    Geom(GeoJSON),
    Int16(i16),
    Int32(i32),
    Str(String),
    Null,
    Unparsed,
}
