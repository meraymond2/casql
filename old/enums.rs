use std::str::FromStr;


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