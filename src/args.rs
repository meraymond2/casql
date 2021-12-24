use crate::cas_err::CasErr;
use crate::configs;
use pico_args::Arguments;
use serde::{Deserialize, Serialize};
use std::fmt;

/*
TODO:
1. help texts for individual connections subcommands
2. decide on conns/config and make the naming consistent
3. possibly split this file into the interface and parsing the args into connection params
4. chop the help texts into reusable blocks
5. if an invalid subcommand is entered, print the valid subcommands
*/

#[derive(Debug)]
pub enum Cmd {
    MainHelp,
    Version,
    Query(ConnectionParams, String),
    QueryHelp,
    ConfigHelp,
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
const VERSION_FLAGS: [&'static str; 2] = ["-V", "--version"];
const HOST_FLAGS: [&'static str; 2] = ["-h", "--host"];
const PORT_FLAGS: [&'static str; 2] = ["-p", "--port"];
const USER_FLAGS: [&'static str; 2] = ["-U", "--username"];
const PASSWORD_FLAGS: [&'static str; 2] = ["-W", "--password"];
const DATABASE_FLAGS: [&'static str; 2] = ["-d", "--dbname"];
const POSTGIS_FLAG: &'static str = "--postgis";
const NAME_FLAGS: [&'static str; 2] = ["-n", "--name"];
const SAVED_CONN_FLAGS: [&'static str; 2] = ["-c", "--conn"];

const VERSION_TEXT: &str = "casql 0.2.0";

const HELP_TEXT_MAIN: &str = "\
casql 0.2.0
Quickly turn SQL into JSON.

USAGE:
    casql <SUBCOMMAND>

FLAGS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    query         Perform a SQL query.
    connection    Operations on saved connections
    help          Print this message or the help of the given subcommand(s)
";

const HELP_TEXT_QUERY: &str = "\
casql query 0.2.0
Perform a SQL query. Connection params can be specified through options, loaded from a saved connection, or a combination of the two.

USAGE:
    casql query [OPTIONS] <QUERY>

ARGS:
    <QUERY>    SQL query to execute

FLAGS:
    -h, --help       Prints help information
    -V, --version    Print version information

OPTIONS:
    -c, --conn <CONNECTION>      Use a saved connection
    -H, --host <HOST>            Database host
    -p, --port <PORT>            Database port
    -d, --dbname <DATABASE>      Database name
    -U, --username <USERNAME>    Database user
    -W, --password <PWD>         Database user’s password
";

const HELP_TEXT_CONNS: &str = "\
casql connection 0.2.0
Operations on saved connections.

USAGE:
    casql connection <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    list        List saved connections
    describe    Describe a saved connection
    save        Save a connection
    delete      Delete a saved connection
    help        Prints this message or the help of the given subcommand(s)
";

pub fn parse_args() -> Result<Cmd, CasErr> {
    let mut args = pico_args::Arguments::from_env();
    if args.contains(VERSION_FLAGS) {
        return Ok(Cmd::Version);
    }
    match args.subcommand()?.as_deref() {
        Some("query") => parse_query(&mut args),
        Some("connection") => parse_conns(&mut args),
        Some("help") => Ok(Cmd::MainHelp),
        Some(other) => Err(CasErr::ArgErr(format!(
            "Unrecognised subcommand: {}",
            other
        ))),
        None => Ok(Cmd::MainHelp),
    }
}

pub fn print_main_help() -> Result<(), CasErr> {
    eprintln!("{}", HELP_TEXT_MAIN);
    Ok(())
}

pub fn print_query_help() -> Result<(), CasErr> {
    eprintln!("{}", HELP_TEXT_QUERY);
    Ok(())
}

pub fn print_conns_help() -> Result<(), CasErr> {
    eprintln!("{}", HELP_TEXT_CONNS);
    Ok(())
}

pub fn print_version() -> Result<(), CasErr> {
    eprintln!("{}", VERSION_TEXT);
    Ok(())
}

fn parse_query(args: &mut Arguments) -> Result<Cmd, CasErr> {
    if args.contains(HELP) {
        return Ok(Cmd::QueryHelp);
    }
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
    Ok(Cmd::Query(params, query))
}

fn parse_conns(args: &mut Arguments) -> Result<Cmd, CasErr> {
    if args.contains(HELP) {
        return Ok(Cmd::ConfigHelp);
    }
    match args.subcommand()?.as_deref() {
        Some("list") => Ok(Cmd::ConfigList),
        Some("save") => parse_conn_save(args),
        Some("delete") => parse_conn_delete(args),
        Some("describe") => parse_conn_describe(args),
        Some(_) => Ok(Cmd::MainHelp),
        None => Ok(Cmd::MainHelp),
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
