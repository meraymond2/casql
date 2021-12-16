// The list of Postgis types that are handled in query results, which excludes internal types.
pub const POSTGIS_TYPES: [&'static str; 5] =
    ["geometry", "geography", "box2d", "box3d", "geometry_dump"];

pub const POSTGIS_TYPE_QUERY: &'static str = "SELECT typname, oid FROM pg_type WHERE typname IN ('geometry', 'geography', 'box2d', 'box3d', 'geometry_dump')";
