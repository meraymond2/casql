use crate::args;
use crate::errors::CasErr;
use clap::ArgMatches;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PartialConnOpts {
  pub host: Option<String>,
  pub password: Option<String>,
  pub database: Option<String>,
  pub port: Option<u16>,
  pub sql_impl: Option<String>,
  pub user: Option<String>,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct ConnOpts {
  pub host: String,
  pub password: Option<String>,
  pub database: String,
  pub port: u16,
  pub sql_impl: String,
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
      user: self.user.or(overlay.user)
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
      incomplete_args => {
          Err(CasErr::IncompleteArgs(incomplete_args))
      }
    }
  }
}

impl From<&ArgMatches<'_>> for PartialConnOpts {
  fn from(matches: &ArgMatches) -> Self {
    let host = matches.value_of(args::HOST);
    let password = matches.value_of(args::PASS);
    let database = matches.value_of(args::DATA);
    let port = match matches.value_of(args::PORT).map(|s| s.parse::<u16>()) {
      Some(Ok(u16)) => Some(u16),
      None => None,
      Some(Err(_)) => panic!("Port must be a number."), // todo, handle better, preferably earlier.
    };
    let sql_impl = matches.value_of(args::IMPL);
    let user = matches.value_of(args::USER);

    PartialConnOpts {
      host: host.map(String::from),
      password: password.map(String::from),
      database: database.map(String::from),
      port,
      sql_impl: sql_impl.map(String::from),
      user: user.map(String::from),
    }
  }
}
