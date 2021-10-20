use std::fmt::{Display, Formatter};
use std::io;
use std::io::ErrorKind;

#[derive(Debug)]
pub enum CasErr {
    JsonErr(String),
    IoBrokenPipe,
    IoConnRefused,
    IoErr(String),
    PostgresErr(String),
}

impl Display for CasErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CasErr::JsonErr(msg) => write!(f, "{}", msg),
            CasErr::IoBrokenPipe => write!(f, ""),
            CasErr::IoConnRefused => write!(f, "Error: could not connect to database"),
            CasErr::IoErr(msg) => write!(f, "{}", msg),
            CasErr::PostgresErr(msg) => write!(f, "{}", msg),
        }
    }
}

impl From<io::Error> for CasErr {
    fn from(err: io::Error) -> Self {
        // eprintln!("{:?}", err.kind());
        match err.kind() {
            ErrorKind::BrokenPipe => CasErr::IoBrokenPipe,
            ErrorKind::ConnectionRefused => CasErr::IoConnRefused,
            _ => CasErr::IoErr(err.to_string()),
        }
    }
}

impl From<serde_json::Error> for CasErr {
    fn from(err: serde_json::Error) -> Self {
        CasErr::JsonErr(err.to_string())
    }
}
