mod args;
mod casable;
mod config;
mod connections;
mod model;
mod pg;

use model::ConnOpts;

fn main() {
    let arg_matches = args::get_args();
    match arg_matches.subcommand() {
        ("query", Some(sub_m)) => {
            let host = sub_m.value_of(args::HOST);
            let password = sub_m.value_of(args::PASS);
            let database = sub_m.value_of(args::DATA);
            let port = match sub_m.value_of(args::PORT).map(|s| s.parse::<u16>()) {
                Some(Ok(u16)) => Some(u16),
                None => None,
                Some(Err(_)) => panic!("Port must be a number."), // todo, handle better, preferably earlier.
            };
            let sql_impl = sub_m.value_of(args::IMPL);
            let user = sub_m.value_of(args::USER);

            let query = String::from(sub_m.value_of("QUERY").unwrap()); // todo, handle

            let conn_opts = match (host, port, database, sql_impl, user) {
                (Some(host), Some(port), Some(database), Some(sql_impl), Some(user)) => ConnOpts {
                    host: String::from(host),
                    password: password.map(|s| String::from(s)),
                    database: String::from(database),
                    port: port,
                    sql_impl: String::from(sql_impl),
                    user: String::from(user),
                },
                _ => panic!("Incomplete args."),
            };

            pg::run_query(query, conn_opts);
        }
        ("save", Some(sub_cmd)) => {
            let opts = model::PartialConnOpts::from(sub_cmd);
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
        _ => println!("Unimplemented..."),
    }
}
