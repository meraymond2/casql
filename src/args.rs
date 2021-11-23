use crate::configs;
use crate::CasErr;
use pico_args::Arguments;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug)]
pub enum Cmd {
    Help,
    Query(ConnectionParams, String),
    ConfigList,
    ConfigSave(PartialConnectionParams, String),
    ConfigDelete(String),
    ConfigDescribe(String),
}

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
    pub postgis: bool,
}

impl fmt::Display for PartialConnectionParams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = toml::to_string(self).map_err(|_| fmt::Error)?;
        write!(f, "{}", text)
    }
}

const HELP: &'static str = "--help";
const HOST_FLAGS: [&'static str; 2] = ["-h", "--host"];
const PORT_FLAGS: [&'static str; 2] = ["-p", "--port"];
const USER_FLAGS: [&'static str; 2] = ["-U", "--username"];
const PASSWORD_FLAGS: [&'static str; 2] = ["-W", "--password"];
const DATABASE_FLAGS: [&'static str; 2] = ["-d", "--dbname"];
const POSTGIS_FLAG: &'static str = "--postgis";
const NAME_FLAGS: [&'static str; 2] = ["-n", "--name"];
const SAVED_CONN_FLAGS: [&'static str; 2] = ["-c", "--conn"];

pub const HELP_TEXT: &str = "\
casql

USAGE:
  casql ...args and stuff

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

pub fn print_help() -> Result<(), CasErr> {
    eprintln!("{}", HELP_TEXT);
    Ok(())
}

fn parse_query(args: &mut Arguments) -> Result<Cmd, CasErr> {
    // Parse the flags
    let host: Option<String> = args.opt_value_from_str(HOST_FLAGS)?;
    let port: Option<u16> = args.opt_value_from_str(PORT_FLAGS)?;
    let user: Option<String> = args.opt_value_from_str(USER_FLAGS)?;
    let password: Option<String> = args.opt_value_from_str(PASSWORD_FLAGS)?;
    let database: Option<String> = args.opt_value_from_str(DATABASE_FLAGS)?;
    let postgis: bool = args.contains(POSTGIS_FLAG);
    let conn_name: Option<String> = args.opt_value_from_str(SAVED_CONN_FLAGS)?;
    // then the query.
    let query = args.free_from_str().map_err(|err| {
        if let pico_args::Error::MissingArgument = err {
            CasErr::ArgErr("Missing required argument <query>".to_owned())
        } else {
            err.into()
        }
    })?;
    let supplied_params = PartialConnectionParams {
        host,
        port,
        user,
        password,
        database,
        postgis,
    };
    let params = match conn_name {
        Some(conn_name) => merge_params(conn_name, supplied_params),
        None => validate_params(supplied_params),
    }?;
    println!("{:?}", params);
    Ok(Cmd::Query(params, query))
}

fn parse_conns(args: &mut Arguments) -> Result<Cmd, CasErr> {
    match args.subcommand().unwrap().as_deref() {
        Some("list") => Ok(Cmd::ConfigList),
        Some("save") => parse_conn_save(args),
        Some("delete") => parse_conn_delete(args),
        Some("describe") => parse_conn_describe(args),
        Some(_) => Ok(Cmd::Help),
        None => Ok(Cmd::Help),
    }
}

fn parse_conn_save(args: &mut Arguments) -> Result<Cmd, CasErr> {
    let name: String = args.value_from_str(NAME_FLAGS)?;
    let host: Option<String> = args.opt_value_from_str(HOST_FLAGS)?;
    let port: Option<u16> = args.opt_value_from_str(PORT_FLAGS)?;
    let user: Option<String> = args.opt_value_from_str(USER_FLAGS)?;
    let password: Option<String> = args.opt_value_from_str(PASSWORD_FLAGS)?;
    let database: Option<String> = args.opt_value_from_str(DATABASE_FLAGS)?;
    let postgis: bool = args.contains(POSTGIS_FLAG);
    Ok(Cmd::ConfigSave(
        PartialConnectionParams {
            host,
            port,
            user,
            password,
            database,
            postgis,
        },
        name,
    ))
}

fn parse_conn_delete(args: &mut Arguments) -> Result<Cmd, CasErr> {
    let name: String = args.value_from_str(NAME_FLAGS)?;
    Ok(Cmd::ConfigDelete(name))
}

fn parse_conn_describe(args: &mut Arguments) -> Result<Cmd, CasErr> {
    let name: String = args.value_from_str(NAME_FLAGS)?;
    Ok(Cmd::ConfigDescribe(name))
}

fn merge_params(
    conn_name: String,
    supplied_params: PartialConnectionParams,
) -> Result<ConnectionParams, CasErr> {
    let loaded_params = configs::load(conn_name)?;
    validate_params(PartialConnectionParams {
        host: supplied_params.host.or(loaded_params.host),
        port: supplied_params.port.or(loaded_params.port),
        user: supplied_params.user.or(loaded_params.user),
        password: supplied_params.password.or(loaded_params.password),
        database: supplied_params.database.or(loaded_params.database),
        postgis: supplied_params.postgis || loaded_params.postgis,
    })
}

fn validate_params(params: PartialConnectionParams) -> Result<ConnectionParams, CasErr> {
    match params {
        PartialConnectionParams {
            host: None,
            user: None,
            ..
        } => Err(CasErr::ArgErr(
            "Missing connection parameters: host, username".to_owned(),
        )),
        PartialConnectionParams { host: None, .. } => Err(CasErr::ArgErr(
            "Missing connection parameters: host".to_owned(),
        )),
        PartialConnectionParams { user: None, .. } => Err(CasErr::ArgErr(
            "Missing connection parameters: username".to_owned(),
        )),
        PartialConnectionParams {
            host: Some(host),
            user: Some(user),
            database,
            password,
            postgis,
            port,
        } => Ok(ConnectionParams {
            host,
            user,
            password,
            database,
            port,
            postgis,
        }),
    }
}
