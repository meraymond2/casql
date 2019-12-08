use std::str::FromStr;


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
