mod args;
mod casable;
mod config;
mod connections;
mod model;
mod pg;

use model::{ConnOpts, PartialConnOpts};

fn main() {
    let arg_matches = args::get_args();
    match arg_matches.subcommand() {
        ("query", Some(sub_m)) => {
            let query = String::from(sub_m.value_of("QUERY").expect("Unreachable."));
            let loaded_opts = if let Some(loaded_name) = sub_m.value_of(args::LOAD) {
                connections::load(loaded_name.to_owned())
            } else {
                PartialConnOpts {
                    host: None,
                    password: None,
                    database: None,
                    port: None,
                    sql_impl: None,
                    user: None,
                }
            };
            let conn_opts = if let Some(connection_string) = sub_m.value_of(args::CSTR) {
                model::ConnectionSpec::Str(connection_string.to_owned())
            } else {
                let arg_opts = PartialConnOpts::from(sub_m);
                match loaded_opts.merge(arg_opts) {
                    PartialConnOpts {
                        host: Some(host),
                        port: Some(port),
                        database: Some(database),
                        sql_impl: Some(sql_impl),
                        user: Some(user),
                        password,
                    } => model::ConnectionSpec::Opts(ConnOpts {
                        host: host,
                        password: password.map(|s| String::from(s)),
                        database: String::from(database),
                        port: port,
                        sql_impl: String::from(sql_impl),
                        user: String::from(user),
                    }),
                    _ => {
                        // TODO: can I get clap to print out the missing valus
                        // in the same way it prints out missing args?
                        eprintln!("Incomplete args.");
                        std::process::exit(1);
                    },
                }
            };

            pg::run_query(query, conn_opts);
        }
        ("save", Some(sub_cmd)) => {
            let opts = PartialConnOpts::from(sub_cmd);
            let name = String::from(sub_cmd.value_of("CONN").unwrap()); // todo, handle
            connections::save(name, opts);
        }
        ("list", _) => {
            connections::list();
        }
        ("describe", Some(sub_cmd)) => {
            let name = String::from(sub_cmd.value_of("CONN").unwrap());
            connections::describe(name);
        }
        ("delete", Some(sub_cmd)) => {
            let name = String::from(sub_cmd.value_of("CONN").unwrap());
            connections::delete(name);
        }
        _ => {} // unreachable
    }
}
