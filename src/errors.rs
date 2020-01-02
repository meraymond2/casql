// use crate::args::PartialConnOpts;
use std::fmt;

#[derive(Debug)]
pub enum CasErr {
  // IncompleteArgs(PartialConnOpts),
  // InvalidPort,
  Unreachable,
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
      // CasErr::InvalidPort => write!(f, "error: That is not a valid port number"),
      CasErr::Unreachable => {
        write!(f, "")
      }
    }
  }
}
