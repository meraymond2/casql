use std::convert::TryInto;
use std::fmt::{Display, Formatter};
use std::io;
use std::io::ErrorKind;

#[derive(Debug)]
pub enum CasErr {
    IoBrokenPipe,
    IoConnRefused,
    IoErr(String),
    PostgresErr(String),
}

impl Display for CasErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CasErr::IoBrokenPipe => write!(f, ""), // ignore SIGPIPEs
            CasErr::IoConnRefused => write!(f, "IO Error: could not connect to database"),
            CasErr::IoErr(msg) => write!(f, "IO Error: {}", msg),
            CasErr::PostgresErr(msg) => write!(f, "Postgres Error: {}", msg),
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
        let io_err: std::io::Error = err.into();
        io_err.into()
    }
}
