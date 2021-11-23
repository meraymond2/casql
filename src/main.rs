mod args;
mod binary_reader;
mod cas_err;
mod configs;
mod postgres;
use crate::args::{Cmd, ConnectionParams};
use crate::cas_err::CasErr;
use postgres::connection::Conn;

mod cas_val;
mod json;

fn main() {
    let args = args::parse_args().unwrap();
    match args {
        Cmd::Help => {
            eprintln!("{}", args::HELP_TEXT);
            std::process::exit(0);
        }
        Cmd::Query(conn_params, query) => match exec_query(conn_params, query) {
            Ok(_) => std::process::exit(0),
            Err(err) => {
                eprintln!("{}", err);
                std::process::exit(1);
            }
        },
        Cmd::ConfigsList => match configs::list() {
            Ok(_) => std::process::exit(0),
            Err(err) => {
                eprintln!("{}", err);
                std::process::exit(1);
            }
        },
    }
}

fn exec_query(params: ConnectionParams, query: String) -> Result<(), CasErr> {
    let mut conn = Conn::connect(params)?;
    conn.query(query, vec![], json::write_json)
}
