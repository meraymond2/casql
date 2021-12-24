use casql::args;
use casql::args::Cmd;
use casql::cas_err::CasErr;
use casql::configs;
use casql::postgres::connection::Conn;
use std::io::BufWriter;

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
        Cmd::MainHelp => args::print_main_help(),
        Cmd::Version => args::print_version(),
        Cmd::Query(conn_params, query) => {
            let stdout = std::io::stdout();
            let handle = stdout.lock();
            let mut out = BufWriter::new(handle);
            let mut conn = Conn::connect(conn_params)?;
            conn.query(query, vec![], &mut out)
        }
        Cmd::QueryHelp => args::print_query_help(),
        Cmd::ConfigList => configs::list(),
        Cmd::ConfigSave(conn_params, name) => configs::save(name, conn_params),
        Cmd::ConfigDelete(name) => configs::delete(name),
        Cmd::ConfigDescribe(name) => configs::describe(name),
        Cmd::ConfigHelp => args::print_conns_help(),
    }?;
    Ok(())
}
