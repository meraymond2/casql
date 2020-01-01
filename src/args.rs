use crate::clap;
use crate::sql_enum::SQLImpl;
use crate::errors::CasErr;
use ::clap::ArgMatches;
use serde_derive::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Debug, Deserialize, Serialize)]
pub struct PartialConnOpts {
  pub host: Option<String>,
  pub password: Option<String>,
  pub database: Option<String>,
  pub port: Option<u16>,
  pub sql_impl: Option<SQLImpl>,
  pub user: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConnOpts {
  pub host: String,
  pub password: Option<String>,
  pub database: String,
  pub port: u16,
  pub sql_impl: SQLImpl,
  pub user: String,
}

pub enum ConnectionSpec {
  Opts(ConnOpts),
  Str(String),
}

impl PartialConnOpts {
  pub fn merge(self, overlay: PartialConnOpts) -> Result<ConnectionSpec, CasErr> {
    let merged = PartialConnOpts {
      host: self.host.or(overlay.host),
      password: self.password.or(overlay.password),
      database: self.database.or(overlay.database),
      port: self.port.or(overlay.port),
      sql_impl: self.sql_impl.or(overlay.sql_impl),
      user: self.user.or(overlay.user),
    };
    match merged {
      PartialConnOpts {
        host: Some(host),
        port: Some(port),
        database: Some(database),
        sql_impl: Some(sql_impl),
        user: Some(user),
        password,
      } => Ok(ConnectionSpec::Opts(ConnOpts {
        host,
        password: password.map(String::from),
        database,
        port,
        sql_impl,
        user,
      })),
      incomplete_args => Err(CasErr::IncompleteArgs(incomplete_args)),
    }
  }
}

impl TryFrom<&ArgMatches<'_>> for PartialConnOpts {
  type Error = CasErr;

  fn try_from(matches: &ArgMatches) -> Result<Self, Self::Error> {
    let host = matches.value_of(clap::HOST);
    let password = matches.value_of(clap::PASS);
    let database = matches.value_of(clap::DATA);
    let port = matches.value_of(clap::PORT).map(|s| s.parse::<u16>());
    let sql_impl = matches.value_of(clap::IMPL).map(SQLImpl::from_str);
    let user = matches.value_of(clap::USER);
    if let Some(Err(_)) = port {
      Err(CasErr::InvalidPort)
    } else {
      Ok(PartialConnOpts {
        host: host.map(String::from),
        password: password.map(String::from),
        database: database.map(String::from),
        port: port.map(|p| p.expect("Error handled above.")),
        sql_impl: sql_impl.map(|p| p.expect("Clap panics! before this happens.")),
        user: user.map(String::from),
      })
    }
  }
}
