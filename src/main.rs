mod args;
mod casable;
mod config;
mod model;
mod pg;

use std::fs::OpenOptions;
use directories::ProjectDirs;
use model::ConnOpts;
use std::io::prelude::*;

fn read_config() -> std::collections::HashMap<std::string::String, model::ConnOpts> {
    let dirs = ProjectDirs::from("", "", "Casql").unwrap();
    let config_path = dirs.config_dir().join("config.toml");

    let mut file = std::fs::File::open(config_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let config: std::collections::HashMap<std::string::String, model::ConnOpts> = toml::from_str(&contents).unwrap();

    config
}

fn write_config(table: std::collections::HashMap<std::string::String, model::ConnOpts>) {
    let dirs = ProjectDirs::from("", "", "Casql").unwrap();
    let config_path = dirs.config_dir().join("config.toml");
    let mut file = OpenOptions::new().write(true).open(config_path).expect("Should be able to open file as writeable");

    let toml = toml::to_string(&table).expect("Config should be tomlable");
    let bytes = toml.into_bytes();
    file.write_all(&bytes).expect("File should write correctly.");
}

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
        ("save", Some(sub_c)) => {
            let host = sub_c.value_of(args::HOST);
            let password = sub_c.value_of(args::PASS);
            let database = sub_c.value_of(args::DATA);
            let port = match sub_c.value_of(args::PORT).map(|s| s.parse::<u16>()) {
                Some(Ok(u16)) => Some(u16),
                None => None,
                Some(Err(_)) => panic!("Port must be a number."), // todo, handle better, preferably earlier.
            };
            let sql_impl = sub_c.value_of(args::IMPL);
            let user = sub_c.value_of(args::USER);

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

            let name = String::from(sub_c.value_of("CONN").unwrap()); // todo, handle

            let mut config_file = read_config();
            config_file.insert(name.clone(), conn_opts);

            write_config(config_file);
            println!("Connection {:?} saved.", name);
        }
        _ => println!("Unimplemented..."),
    }
}
