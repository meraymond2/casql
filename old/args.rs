use clap::{crate_authors, crate_version, App, Arg, ArgMatches, SubCommand};
use crate::enums::{MYSQL, POSTGRESQL};

pub const HOST: &str = "HOST";
pub const PORT: &str = "PORT";
pub const USER: &str = "USER";
pub const PASS: &str = "PWD";
pub const DATA: &str = "DATA";
pub const IMPL: &str = "IMPL";
pub const LOAD: &str = "LOAD";
pub const CSTR: &str = "CONN_STRING";
pub const CONN: &str = "CONN";

fn build_clap_app<'a>() -> App<'a, 'a> {
  let host = Arg::with_name(HOST)
    .short("h")
    .long("host")
    .help("Database host")
    .takes_value(true);
  let port = Arg::with_name(PORT)
    .short("p")
    .long("port")
    .help("Database port")
    .takes_value(true)
    .default_value_ifs(&[
      (IMPL, Some(POSTGRESQL), "5432"),
      (IMPL, Some(MYSQL), "3306"),
    ]);
  let user = Arg::with_name(USER)
    .short("u")
    .long("user")
    .help("Database user")
    .takes_value(true);
  let password = Arg::with_name(PASS)
    .short("w")
    .long("password")
    .help("Database user’s password")
    .takes_value(true);
  let database = Arg::with_name(DATA)
    .short("d")
    .long("database")
    .help("Database name")
    .takes_value(true);
  let sql_impl = Arg::with_name(IMPL)
    .short("i")
    .long("implementation")
    .help("SQL implementation")
    .takes_value(true)
    .possible_values(&[POSTGRESQL, MYSQL]);
  let load = Arg::with_name(LOAD)
    .short("l")
    .long("load")
    .help("Load a saved connection")
    .takes_value(true);
  let conn_string = Arg::with_name(CSTR)
    .short("s")
    .long("conn-string")
    .help("A valid SQL server connection string")
    .takes_value(true);
  let conn = Arg::with_name(CONN).help("The name of a connection");

  let query = SubCommand::with_name("query")
    .about("Execute a SQL query")
    .arg(Arg::from(&host).required_unless_one(&[LOAD, CSTR]))
    .arg(Arg::from(&user).required_unless_one(&[LOAD, CSTR]))
    .arg(Arg::from(&sql_impl).required_unless_one(&[LOAD, CSTR]))
    .arg(Arg::from(&port))
    .arg(Arg::from(&password))
    .arg(Arg::from(&database))
    .arg(Arg::from(&load))
    .arg(Arg::from(&conn_string).conflicts_with_all(&[HOST, PORT, USER, PASS, IMPL, LOAD]))
    .arg(
      Arg::with_name("QUERY")
        .takes_value(true)
        .help("SQL query to execute")
        .required(true),
    );

  let list = SubCommand::with_name("list").about("List saved connections");

  let describe = SubCommand::with_name("describe")
    .about("Describe a saved connection")
    .arg(Arg::from(&conn).required(true));

  let save = SubCommand::with_name("save")
    .about("Save a connection")
    .arg(Arg::from(&host))
    .arg(Arg::from(&user))
    .arg(Arg::from(&sql_impl))
    .arg(Arg::from(&port))
    .arg(Arg::from(&database))
    .arg(Arg::from(&password))
    .arg(Arg::from(&conn_string).conflicts_with_all(&[HOST, PORT, USER, PASS, IMPL]))
    .arg(Arg::from(&conn).required(true));

  let delete = SubCommand::with_name("delete")
    .about("Delete a saved connection")
    .arg(Arg::from(&conn).required(true));

  App::new("casql")
    .author(crate_authors!())
    .version(crate_version!())
    .about("Quickly turn SQL into JSON.")
    .help_short("H")
    .version_short("v")
    .subcommand(query.display_order(1))
    .subcommand(list.display_order(2))
    .subcommand(describe.display_order(3))
    .subcommand(save.display_order(4))
    .subcommand(delete.display_order(5))
}

pub fn get_args<'a>() -> ArgMatches<'a> {
  let app = build_clap_app();
  app.get_matches()
}