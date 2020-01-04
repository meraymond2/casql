use crate::connections;
use crate::errors::CasErr;
use crate::mysql;
use crate::opts::PartialConnOpts;
use crate::postgres;
use crate::sql_enum::SQLImpl;
use std::convert::TryFrom;

#[derive(Debug)]
pub struct ConnOpts {
  pub database: String,
  pub host: String,
  pub password: Option<String>,
  pub port: u16,
  pub sql_impl: SQLImpl,
  pub user: String,
}

impl TryFrom<PartialConnOpts> for ConnOpts {
  type Error = CasErr;

  fn try_from(partial_opts: PartialConnOpts) -> Result<Self, Self::Error> {
    match partial_opts {
      PartialConnOpts {
        database: Some(database),
        host: Some(host),
        password,
        port: Some(port),
        sql_impl: Some(sql_impl),
        user: Some(user),
      } => Ok(ConnOpts {
        database,
        host,
        password,
        port,
        sql_impl,
        user,
      }),
      _ => Err(CasErr::IncompleteArgs(partial_opts)),
    }
  }
}

pub enum ConnectionSpec {
  Opts(ConnOpts),
  Str(String),
}

pub fn exec(
  conn_name: Option<String>,
  conn_str: Option<String>,
  opts: PartialConnOpts,
  query: String,
) -> Result<(), CasErr> {
  match (conn_name, conn_str) {
    (_, Some(conn_str)) => {
      if conn_str.starts_with("postgres") {
        postgres::exec(query, ConnectionSpec::Str(conn_str))
      } else if conn_str.starts_with("mysql") {
        mysql::exec(query, ConnectionSpec::Str(conn_str))
      } else {
        Err(CasErr::InvalidConnectionString)
      }
    }
    (Some(conn_name), _) => {
      let loaded_opts = connections::load(conn_name)?;
      let complete_opts = ConnOpts::try_from(loaded_opts.merge(opts))?;
      match complete_opts.sql_impl {
        SQLImpl::MySQL => mysql::exec(query, ConnectionSpec::Opts(complete_opts)),
        SQLImpl::PostgreSQL => postgres::exec(query, ConnectionSpec::Opts(complete_opts)),
      }
    }
    (None, None) => {
      let complete_opts = ConnOpts::try_from(opts)?;
      match complete_opts.sql_impl {
        SQLImpl::MySQL => mysql::exec(query, ConnectionSpec::Opts(complete_opts)),
        SQLImpl::PostgreSQL => postgres::exec(query, ConnectionSpec::Opts(complete_opts)),
      }
    }
  }
}
