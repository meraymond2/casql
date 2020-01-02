mod opts;
mod connections;
mod errors;
mod sql_enum;

use crate::opts::{Connection, Opt};

fn main() {
  match Opt::parse() {
    Opt::Connection(subcmd) => match subcmd {
      Connection::Save(opts) => println!("Opts: {:?}", opts),
      Connection::List => connections::list(),
      Connection::Describe { conn_name } => println!("Stuff: {:?}", conn_name),
    },
  }
}
