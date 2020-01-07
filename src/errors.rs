use crate::opts::PartialConnOpts;
use mysql;
use std::error;
use std::fmt;
use std::io;
use std::io::ErrorKind;

#[derive(Debug)]
pub enum CasErr {
  IncompleteArgs(PartialConnOpts),
  ConnNotFound,
  FilePermissions,
  InvalidConfigToml(String),
  InvalidConnectionString,
  NoHomeDir,
  InvalidSQLUrl,
  UnknownIO(String),
  UnknownSQL(String),
  Unreachable,
}

impl error::Error for CasErr {}

impl From<io::Error> for CasErr {
  fn from(err: io::Error) -> Self {
    match err {
      ref e if e.kind() == ErrorKind::PermissionDenied => return CasErr::FilePermissions,
      e => CasErr::UnknownIO(format!("{}", e)),
    }
  }
}

impl From<mysql::error::UrlError> for CasErr {
  fn from(_err: mysql::error::UrlError) -> Self {
    CasErr::InvalidSQLUrl
  }
}

impl From<mysql::error::Error> for CasErr {
  fn from(err: mysql::error::Error) -> Self {
    CasErr::UnknownSQL(format!("{}", err))
  }
}

impl fmt::Display for CasErr {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      CasErr::IncompleteArgs(opts) => {
        let database = match opts.database {
          Some(_) => "",
          None => "    --database\n",
        };
        let host = match opts.host {
          Some(_) => "",
          None => "    --host\n",
        };
        let port = match opts.port {
          Some(_) => "",
          None => "    --port\n",
        };
        let sql_impl = match opts.sql_impl {
          Some(_) => "",
          None => "    --implementation\n",
        };
        let user = match opts.user {
          Some(_) => "",
          None => "    --user\n",
        };
        write!(f, "error: The following required arguments were not provided:\n{}{}{}{}{}\nFor more information try --help", database, host, port, sql_impl, user)
      }
      CasErr::ConnNotFound => write!(f, "error: That connection was not found"),
      CasErr::FilePermissions => write!(
        f,
        "error: casql does not have permission to write to its config files"
      ),
      CasErr::InvalidConfigToml(reason) => write!(f, "error: {}", reason),
      CasErr::InvalidConnectionString => write!(f, "error: Connection string could not be associated with a SQL backend"),
      CasErr::InvalidSQLUrl => write!(f, "error: Could not connect to the database with that SQL url"),
      CasErr::NoHomeDir => write!(f, "error: Could not determine user’s home directory"),
      CasErr::UnknownIO(reason) => write!(f, "error: There was an unexpected IO error: {}", reason),
      CasErr::UnknownSQL(reason) => write!(f, "error: There was a problem connecting to the database: {}", reason),
      CasErr::Unreachable => write!(f, ""),
    }
  }
}
