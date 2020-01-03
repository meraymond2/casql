use crate::errors::CasErr;
use crate::opts::PartialConnOpts;
use crate::sql_enum::SQLImpl;

#[derive(Debug)]
pub struct ConnOpts {
  pub database: String,
  pub host: String,
  pub password: Option<String>,
  pub port: u16,
  pub sql_impl: SQLImpl,
  pub user: String,
}

pub enum ConnectionSpec {
  Opts(ConnOpts),
  Str(String),
}

// TODO: write a try_from for partial to complete ConnOpts
// TODO: re-implement the load-connection method
// TODO: re-implement the merge method
// There will probably be some repetition here, I think it's ok for now

pub fn exec(
  opts: PartialConnOpts,
  to_load_opt: Option<String>,
  conn_str_opt: Option<String>,
) -> Result<(), CasErr> {
  let conn_spec = match (to_load_opt, conn_str_opt) {
    (None, Some(conn_str)) => ConnectionSpec::Str(conn_str),
    (Some(conn_to_load), None) => ConnectionSpec::Str("?".to_owned()),
    (None, None) => ConnectionSpec::Str("TODO".to_owned()),
    _ => return Err(CasErr::Unreachable),
  };
  Ok(())
}
