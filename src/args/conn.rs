use std::str::FromStr;

// TODO, why can't I call this from main??

// TODO: this is in the wrong place, this is just a temporary solution in order
// to be able to access it from args
pub const POSTGRESQL: &str = "postgres";
pub const MYSQL: &str = "mysql";

pub enum SQLImpl {
  MySQL,
  PostgreSQL,
}

impl FromStr for SQLImpl {
  // TODO: what sort of error should this be?
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      MYSQL => Ok(SQLImpl::MySQL),
      POSTGRESQL => Ok(SQLImpl::PostgreSQL),
      _ => Err("Invalid match."),
    }
  }
}

struct PartialConnOpts {
  host: Option<String>,
  password: Option<String>,
  port: Option<usize>,
  sql_impl: Option<SQLImpl>,
  user: Option<String>,
}

struct ConnOpts {
  host: String, // I don't think this can be more specific
  password: Option<String>,
  port: usize, // This can possibly be more specific
  sql_impl: SQLImpl,
  user: String,
}

enum ConnectionSpec {
  Opts(ConnOpts),
  Str(String),
}
