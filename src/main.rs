mod args;
mod casable;
mod model;
mod pg;

use model::ConnOpts;

fn main() {
    let arg_matches = args::get_args();
    match arg_matches.subcommand() {
        ("query", Some(sub_m)) => {
            let host = sub_m.value_of(args::HOST);
            let password = sub_m.value_of(args::PASS);
            let port = match sub_m.value_of(args::PORT).map(|s| s.parse::<u16>()) {
                Some(Ok(u16)) => Some(u16),
                None => None,
                Some(Err(_)) => panic!("Port must be a number."), // todo, handle better, preferably earlier.
            };
            let sql_impl = sub_m.value_of(args::IMPL);
            let user = sub_m.value_of(args::USER);

            let query = String::from(sub_m.value_of("QUERY").unwrap()); // todo, handle

            let conn_opts = match (host, port, sql_impl, user) {
                (Some(host), Some(port), Some(sql_impl), Some(user)) => ConnOpts {
                    host: String::from(host),
                    password: password.map(|s| String::from(s)),
                    port: port,
                    sql_impl: String::from(sql_impl),
                    user: String::from(user),
                },
                _ => panic!("Incomplete args."),
            };

            pg::run_query(query, conn_opts);
        }
        _ => println!("Unimplemented..."),
    }
}
