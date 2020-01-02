mod connections;
mod errors;
mod opts;
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
  };

  std::process::exit(match res {
    Ok(_) => 0,
    Err(e) => {
      eprintln!("{}", e);
      1
    }
  })
}
