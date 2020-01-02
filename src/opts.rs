use crate::sql_enum::SQLImpl;
use clap::Clap;
use serde_derive::{Deserialize, Serialize};

#[derive(Clap, Debug, Deserialize, Serialize)]
pub struct PartialConnOpts {
    #[clap(name = "HOST", long = "host", short = "H", help = "Database host")]
    host: Option<String>,

    #[clap(name = "PORT", long = "port", short = "p", help = "Database port")]
    port: Option<u16>,

    #[clap(name = "USER", long = "user", short = "u", help = "Database user")]
    user: Option<String>,

    #[clap(
        name = "PWD",
        long = "password",
        short = "w",
        help = "Database user’s password"
    )]
    password: Option<String>,

    #[clap(
        name = "DATABASE",
        long = "database",
        short = "d",
        help = "Database name"
    )]
    database: Option<String>,

    #[clap(
        name = "SQL_IMPL",
        long = "implementation",
        short = "i",
        help = "SQL implementation"
    )]
    sql_impl: Option<SQLImpl>,
}

#[derive(Clap, Debug)]
pub enum Connection {
    #[clap(name = "save", about = "Save a connection")]
    Save {
        #[clap(name = "NAME")]
        conn_name: String,

        #[clap(flatten)]
        opts: PartialConnOpts,
    },

    #[clap(name = "list", about = "List saved connections")]
    List,

    #[clap(name = "describe", about = "Describe a saved connection")]
    Describe {
        #[clap(name = "NAME")]
        conn_name: String,
    },
}

#[derive(Clap, Debug)]
#[clap(about = "Quickly turn SQL into JSON.")]
pub enum Opt {
    #[clap(name = "connection", about = "Operations on saved connections")]
    Connection(Connection),
}
