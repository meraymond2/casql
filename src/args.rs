use crate::CasErr;
use pico_args::Arguments;
use serde::{Deserialize, Serialize};

// TODO:
// 1. help text
// 2. other sub-commands
// 3. once connection commands are done, merge loaded params with provided
#[derive(Debug)]
pub struct ConnectionParams {
    pub host: String,
    pub user: String,
    pub password: Option<String>,
    pub database: Option<String>,
    pub port: Option<u16>,
    pub postgis: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PartialConnectionParams {
    pub host: Option<String>,
    pub user: Option<String>,
    pub password: Option<String>,
    pub database: Option<String>,
    pub port: Option<u16>,
    pub postgis: Option<bool>,
}

pub const HELP_TEXT: &str = "\
casql

USAGE:
  casql [OPTIONS] --number NUMBER [INPUT]

FLAGS:
  -h, --help            Prints help information

OPTIONS:
  --number NUMBER       Sets a number
  --opt-number NUMBER   Sets an optional number
  --width WIDTH         Sets width [default: 10]
  --output PATH         Sets an output path

ARGS:
  <INPUT>
";

const HELP: &'static str = "--help";
const HOST_FLAGS: [&'static str; 2] = ["-h", "--host"];
const PORT_FLAGS: [&'static str; 2] = ["-p", "--port"];
const USER_FLAGS: [&'static str; 2] = ["-U", "--username"];
const PASSWORD_FLAGS: [&'static str; 2] = ["-W", "--password"];
const DATABASE_FLAGS: [&'static str; 2] = ["-d", "--dbname"];

#[derive(Debug)]
pub enum Cmd {
    Help,
    Query(ConnectionParams, String),
    ConfigsList,
}

pub fn parse_args() -> Result<Cmd, CasErr> {
    let mut args = pico_args::Arguments::from_env();
    if args.contains(HELP) {
        return Ok(Cmd::Help);
    }
    match args.subcommand().unwrap().as_deref() {
        Some("query") => parse_query(&mut args),
        Some("conns") => parse_conns(&mut args),
        Some(other) => Err(CasErr::ArgErr(format!("Unrecognised command: {}", other))),
        None => Ok(Cmd::Help),
    }
}

fn parse_query(args: &mut Arguments) -> Result<Cmd, CasErr> {
    // Parse the flags
    let host: String = args.value_from_str(HOST_FLAGS)?;
    let port: Option<u16> = args.opt_value_from_str(PORT_FLAGS)?;
    let user: String = args.value_from_str(USER_FLAGS)?;
    let password: Option<String> = args.opt_value_from_str(PASSWORD_FLAGS)?;
    let database: Option<String> = args.opt_value_from_str(DATABASE_FLAGS)?;
    // then the query.
    let query = args.free_from_str().map_err(|err| {
        if let pico_args::Error::MissingArgument = err {
            CasErr::ArgErr("Missing required argument <query>".to_owned())
        } else {
            err.into()
        }
    })?;
    Ok(Cmd::Query(
        ConnectionParams {
            host,
            port,
            user,
            password,
            database,
            postgis: false,
        },
        query,
    ))
}

fn parse_conns(args: &mut Arguments) -> Result<Cmd, CasErr> {
    match args.subcommand().unwrap().as_deref() {
        Some("list") => Ok(Cmd::ConfigsList),
        None => Ok(Cmd::Help),
        _ => unimplemented!(),
    }
}
