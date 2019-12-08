use crate::args;
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
      host: host.map(|s| String::from(s)),
      password: password.map(|s| String::from(s)),
      database: database.map(|s| String::from(s)),
      port: port,
      sql_impl: sql_impl.map(|s| String::from(s)),
      user: user.map(|s| String::from(s)),
    }
  }
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

// enum ConnectionSpec {
//   Opts(ConnOpts),
//   Str(String),
// }
