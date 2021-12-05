use pico_args::Error;
use std::fmt::{Display, Formatter};
use std::io;
use std::io::ErrorKind;

#[derive(Debug)]
pub enum CasErr {
    ArgErr(String),
    ConfigsErr(String),
    IoBrokenPipe,
    IoConnRefused,
    IoErr(String),
    PostgresErr(String),
    Utf8Err(String),
}

impl Display for CasErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CasErr::ArgErr(msg) => write!(f, "{}", msg),
            CasErr::ConfigsErr(msg) => write!(f, "Configs Error: {}", msg),
            CasErr::IoBrokenPipe => write!(f, ""), // ignore SIGPIPEs
            CasErr::IoConnRefused => write!(f, "IO Error: could not connect to database"),
            CasErr::IoErr(msg) => write!(f, "IO Error: {}", msg),
            CasErr::PostgresErr(msg) => write!(f, "Postgres Error: {}", msg),
            CasErr::Utf8Err(msg) => write!(f, "UTF-8 Error: {}", msg),
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

impl From<std::str::Utf8Error> for CasErr {
    fn from(err: std::str::Utf8Error) -> Self {
        CasErr::Utf8Err(err.to_string())
    }
}

impl From<serde_json::Error> for CasErr {
    fn from(err: serde_json::Error) -> Self {
        let io_err: std::io::Error = err.into();
        io_err.into()
    }
}

impl From<pico_args::Error> for CasErr {
    fn from(err: pico_args::Error) -> Self {
        match err {
            Error::NonUtf8Argument => {
                CasErr::ArgErr("Only UTF-8 arguments are supported.".to_owned())
            }
            Error::MissingArgument => CasErr::ArgErr("Missing required argument.".to_owned()),
            Error::MissingOption(opt) => {
                CasErr::ArgErr(format!("Missing required option {:?}.", opt))
            }
            Error::OptionWithoutAValue(opt) => {
                CasErr::ArgErr(format!("Missing value for option {}.", opt))
            }
            Error::Utf8ArgumentParsingFailed { value, cause } => {
                CasErr::ArgErr(format!("Failed to parse {}: {}", value, cause))
            }
            Error::ArgumentParsingFailed { cause } => {
                CasErr::ArgErr(format!("Failed to parse argument: {}", cause))
            }
        }
    }
}
