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
    match run() {
        Ok(_) => std::process::exit(0),
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}

fn run() -> Result<(), CasErr> {
    let args = args::parse_args()?;
    match args {
        Cmd::Help => args::print_help(),
        Cmd::Query(conn_params, query) => exec_query(conn_params, query),
        Cmd::ConfigList => configs::list(),
        Cmd::ConfigSave(conn_params, name) => configs::save(name, conn_params),
        Cmd::ConfigDelete(name) => configs::delete(name),
        Cmd::ConfigDescribe(name) => configs::describe(name),
    }?;
    Ok(())
}

fn exec_query(params: ConnectionParams, query: String) -> Result<(), CasErr> {
    let mut conn = Conn::connect(params)?;
    conn.query(query, vec![], json::write_json)
}
