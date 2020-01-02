// mod args;
mod opts;
// mod connections;
// mod errors;
// mod sql_enum;

use crate::opts::{Connection, Opt};
// use crate::args::{ConnectionSpec, PartialConnOpts};
// use crate::clap::get_args;
// use std::convert::TryFrom;

fn main() {
  match Opt::parse() {
    Opt::Connection(subcmd) => match subcmd {
      Connection::List => println!("They want a list."),
      Connection::Describe { conn_name } => println!("Stuff: {:?}", conn_name),
    },
  }
  // let arg_matches = get_args();
  // match arg_matches.subcommand() {
  //   ("query", Some(sub_m)) => {
  //     let query = String::from(sub_m.value_of("QUERY").expect("Unreachable."));
  //     let loaded_opts = if let Some(loaded_name) = sub_m.value_of(clap::LOAD) {
  //       connections::load(loaded_name)
  //     } else {
  //       PartialConnOpts {
  //         host: None,
  //         password: None,
  //         database: None,
  //         port: None,
  //         sql_impl: None,
  //         user: None,
  //       }
  //     };
  //     let conn_opts = if let Some(connection_string) = sub_m.value_of(clap::CSTR) {
  //       args::ConnectionSpec::Str(connection_string.to_owned())
  //     } else {
  //       let arg_opts = PartialConnOpts::try_from(sub_m).expect("TODO: Handle this");
  //       match loaded_opts.merge(arg_opts) {
  //         Ok(conn_opts) => conn_opts,
  //         Err(incomplete_args_err) => {
  //           eprintln!("{}", incomplete_args_err);
  //           std::process::exit(1);
  //         }
  //       }
  //     };
  //     println!("TODO: query stuff")
  //     // pg::run_query(query, conn_opts);
  //   }
  //   ("save", Some(sub_cmd)) => {
  //     if let Some(name) = sub_cmd.value_of("CONN") {
  //       let opts = PartialConnOpts::try_from(sub_cmd).expect("TODO: Handle this");
  //       connections::save(name, opts);
  //     }
  //   }
  //   ("list", _) => {
  //     connections::list();
  //   }
  //   ("describe", Some(sub_cmd)) => {
  //     if let Some(name) = sub_cmd.value_of("CONN") {
  //       connections::describe(name);
  //     }
  //   }
  //   ("delete", Some(sub_cmd)) => {
  //     if let Some(name) = sub_cmd.value_of("CONN") {
  //       connections::delete(name);
  //     }
  //   }
  //   _ => {} // unreachable
  // }
}
