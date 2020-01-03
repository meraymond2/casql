mod connections;
mod errors;
mod opts;
mod query;
mod sql_enum;

use crate::opts::{Connection, Opt};

fn main() {
  let res = match Opt::parse() {
    Opt::Connection(subcmd) => match subcmd {
      Connection::Save { conn_name, opts } => connections::save(conn_name, opts),
      Connection::List => connections::list(),
      Connection::Describe { conn_name } => connections::describe(conn_name),
      Connection::Delete { conn_name } => connections::delete(conn_name),
    },
    Opt::Query { conn_name: _, conn_str: Some(conn_str), opts: _ } => query::exec_with_conn_str(conn_str),
    Opt::Query { conn_name: None, conn_str: None, opts } => {query::exec_with_opts(opts)},
    Opt::Query { conn_name: Some(conn_name), conn_str: None, opts } => {query::exec_with_loaded_opts(opts, conn_name)},
  };

  std::process::exit(match res {
    Ok(_) => 0,
    Err(e) => {
      eprintln!("{}", e);
      1
    }
  })
}
