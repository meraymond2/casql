use crate::errors::CasErr;
use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, Serializer};
use std::str::FromStr;

pub const POSTGRESQL: &str = "postgres";
pub const MYSQL: &str = "mysql";

#[derive(Debug)]
pub enum SQLImpl {
  MySQL,
  PostgreSQL,
}

impl FromStr for SQLImpl {
  type Err = CasErr;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      MYSQL => Ok(SQLImpl::MySQL),
      POSTGRESQL => Ok(SQLImpl::PostgreSQL),
      // Clap will panic! first.
      _ => Err(CasErr::Unreachable),
    }
  }
}

struct SqlImplVisitor;

impl Serialize for SQLImpl {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    match self {
      SQLImpl::MySQL => serializer.serialize_str(MYSQL),
      SQLImpl::PostgreSQL => serializer.serialize_str(POSTGRESQL),
    }
  }
}

impl<'de> Visitor<'de> for SqlImplVisitor {
  type Value = SQLImpl;

  fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
    formatter.write_str("a string of \"mysql\" or \"postgres\".")
  }

  fn visit_str<E>(self, value: &str) -> Result<SQLImpl, E>
  where
    E: de::Error,
  {
    match value {
      MYSQL => Ok(SQLImpl::MySQL),
      POSTGRESQL => Ok(SQLImpl::PostgreSQL),
      other => Err(E::custom(format!("Unknown string value: {}", other))),
    }
  }
}

impl<'de> Deserialize<'de> for SQLImpl {
  fn deserialize<D>(deserializer: D) -> Result<SQLImpl, D::Error>
  where
    D: Deserializer<'de>,
  {
    deserializer.deserialize_str(SqlImplVisitor)
  }
}
