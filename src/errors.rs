use crate::opts::PartialConnOpts;
use postgres;
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
    NoHomeDir,
    UnknownIO(String),
    UnknownSQL(String),
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

impl From<postgres::error::Error> for CasErr {
    fn from(err: postgres::error::Error) -> Self {
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
                let user = match opts.user {
                    Some(_) => "",
                    None => "    --user\n",
                };
                write!(f, "error: The following required arguments were not provided:\n{}{}{}{}\nFor more information try --help", database, host, port, user)
            }
            CasErr::ConnNotFound => write!(f, "error: That connection was not found"),
            CasErr::FilePermissions => write!(
                f,
                "error: casql does not have permission to write to its config files"
            ),
            CasErr::InvalidConfigToml(reason) => write!(f, "error: {}", reason),
            CasErr::NoHomeDir => write!(f, "error: Could not determine user’s home directory"),
            CasErr::UnknownIO(reason) => write!(f, "error: There was an unexpected IO error: {}", reason),
            CasErr::UnknownSQL(reason) => write!(f, "error: There was a problem connecting to the database: {}", reason),
        }
    }
}
