mod casable;
mod connections;
mod errors;
mod mysql;
mod opts;
mod postgres;
mod query;
mod sql_enum;

use crate::opts::{Connection, Opt};

fn main() {
  let res = match opts::parse_opts() {
    Opt::Connection(subcmd) => match subcmd {
      Connection::Save { conn_name, opts } => connections::save(conn_name, opts),
      Connection::List => connections::list(),
      Connection::Describe { conn_name } => connections::describe(conn_name),
      Connection::Delete { conn_name } => connections::delete(conn_name),
    },
    Opt::Query {
      conn_name,
      conn_str,
      opts,
      query,
    } => query::exec(conn_name, conn_str, opts, query),
  };

  std::process::exit(match res {
    Ok(_) => 0,
    Err(e) => {
      eprintln!("{}", e);
      1
    }
  })
}
