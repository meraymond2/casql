use crate::connections;
use crate::errors::CasErr;
use crate::opts::PartialConnOpts;
use crate::mysql;
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

fn exec(query: String, conn_spec: ConnectionSpec) -> Result<(), CasErr> {
  match conn_spec {
    ConnectionSpec::Opts(ConnOpts {
      sql_impl: SQLImpl::PostgreSQL,
      ..
    }) => postgres::exec(query, conn_spec),
    ConnectionSpec::Opts(ConnOpts {
      sql_impl: SQLImpl::MySQL,
      ..
    }) => mysql::exec(query, conn_spec),
    ConnectionSpec::Str(conn_string) => {
      Ok(())
    }
  }
}

pub fn exec_with_opts(query: String, opts: PartialConnOpts) -> Result<(), CasErr> {
  let complete_opts = ConnOpts::try_from(opts)?;
  exec(query, ConnectionSpec::Opts(complete_opts))
}

pub fn exec_with_loaded_opts(
  query: String,
  opts: PartialConnOpts,
  conn_name: String,
) -> Result<(), CasErr> {
  let loaded_opts = connections::load(conn_name)?;
  let complete_opts = ConnOpts::try_from(loaded_opts.merge(opts))?;
  exec(query, ConnectionSpec::Opts(complete_opts))
}

pub fn exec_with_conn_str(query: String, conn_str: String) -> Result<(), CasErr> {
  exec(query, ConnectionSpec::Str(conn_str))
}
