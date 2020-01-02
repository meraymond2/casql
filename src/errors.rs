// use crate::args::PartialConnOpts;
use std::error;
use std::fmt;
use std::io;
use std::io::ErrorKind;

#[derive(Debug)]
pub enum CasErr {
  // IncompleteArgs(PartialConnOpts),
  ConnNotFound,
  FilePermissions,
  InvalidConfigToml(String),
  NoHomeDir,
  UnknownIO(String),
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

impl fmt::Display for CasErr {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      // CasErr::IncompleteArgs(opts) => {
      //   let host = if opts.host.is_none() {
      //     "    --host\n"
      //   } else {
      //     ""
      //   };
      //   let database = if opts.database.is_none() {
      //     "    --database\n"
      //   } else {
      //     ""
      //   };
      //   let port = if opts.port.is_none() {
      //     "    --port\n"
      //   } else {
      //     ""
      //   };
      //   let sql_impl = if opts.sql_impl.is_none() {
      //     "    --sql_impl\n"
      //   } else {
      //     ""
      //   };
      //   let user = if opts.user.is_none() {
      //     "    --user\n"
      //   } else {
      //     ""
      //   };
      //   write!(f, "error: The following required arguments were not provided:\n{}{}{}{}{}\nFor more information try --help", host, database, port, sql_impl, user)
      // }
      CasErr::ConnNotFound => write!(f, "error: That connection was not found"),
      CasErr::FilePermissions => write!(
        f,
        "error: casql does not have permission to write to its config files"
      ),
      CasErr::InvalidConfigToml(reason) => write!(f, "error: {}", reason),
      CasErr::NoHomeDir => write!(f, "error: Could not determine user’s home directory"),
      CasErr::UnknownIO(reason) => write!(f, "error: There was an unexpected IO error: {}", reason),
      CasErr::Unreachable => write!(f, ""),
    }
  }
}
