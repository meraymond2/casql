mod connections;
mod errors;
mod opts;
mod sql_enum;

use crate::opts::{Connection, Opt};

fn main() {
  let res = match Opt::parse() {
    Opt::Connection(subcmd) => match subcmd {
      // Connection::Save(opts) => println!("Opts: {:?}", opts),
      Connection::Save(_opts) => Ok(()),
      Connection::List => connections::list(),
      Connection::Describe { conn_name } => connections::describe(conn_name),
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
