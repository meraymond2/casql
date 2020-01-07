use crate::sql_enum::{SQLImpl, MYSQL, POSTGRESQL};
use clap::Clap;
use serde_derive::{Deserialize, Serialize};
use std::fmt;

#[derive(Clap, Debug, Deserialize, Serialize)]
pub struct PartialConnOpts {
    #[clap(name = "HOST", long = "host", short = "H", help = "Database host")]
    pub host: Option<String>,

    #[clap(
        name = "PORT",
        long = "port",
        short = "p",
        help = "Database port",
        default_value_ifs = &[
            ("SQL_IMPL", Some(POSTGRESQL), "5432"),
            ("SQL_IMPL", Some(MYSQL), "3306"),
        ]
    )]
    pub port: Option<u16>,

    #[clap(name = "USER", long = "user", short = "u", help = "Database user")]
    pub user: Option<String>,

    #[clap(
        name = "PWD",
        long = "password",
        short = "w",
        help = "Database user’s password"
    )]
    pub password: Option<String>,

    #[clap(
        name = "DATABASE",
        long = "database",
        short = "d",
        help = "Database name"
    )]
    pub database: Option<String>,

    #[clap(
        name = "SQL_IMPL",
        long = "implementation",
        short = "i",
        help = "SQL implementation",
        possible_values = &[POSTGRESQL, MYSQL]
    )]
    pub sql_impl: Option<SQLImpl>,
}

impl PartialConnOpts {
  pub fn merge(self, overlay: PartialConnOpts) -> PartialConnOpts {
    PartialConnOpts {
      host: overlay.host.or(self.host),
      password: overlay.password.or(self.password),
      database: overlay.database.or(self.database),
      port: overlay.port.or(self.port),
      sql_impl: overlay.sql_impl.or(self.sql_impl),
      user: overlay.user.or(self.user),
    }
  }
}

impl fmt::Display for PartialConnOpts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = toml::to_string(self).map_err(|_| fmt::Error)?;
        write!(f, "{}", text)
    }
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

    #[clap(name = "delete", about = "Delete a saved connection")]
    Delete {
        #[clap(name = "NAME")]
        conn_name: String,
    },
}

#[derive(Clap, Debug)]
#[clap(about = "Quickly turn SQL into JSON.")]
pub enum Opt {
    #[clap(name = "connection", about = "Operations on saved connections")]
    Connection(Connection),

    #[clap(
        name = "query",
        about = "Execute a SQL query. Connection can be specified as a connection string or individual options."
    )]
    Query {
        #[clap(flatten)]
        opts: PartialConnOpts,

        #[clap(
            name = "NAME",
            long = "load",
            short = "l",
            help = "Load a saved connection"
        )]
        conn_name: Option<String>,

        #[clap(
            name = "CONN_STR",
            long = "conn-str",
            short = "s",
            help = "SQL server connection string",
            // TODO: this outputs the wrong error msg,
            // might be a bug in Clap?
            conflicts_with_all = &[
                "HOST",
                "PORT",
                "USER",
                "PWD",
                "SQL_IMPL",
                "NAME"
            ]
        )]
        conn_str: Option<String>,

        #[clap(name = "QUERY", help = "SQL query to execute")]
        query: String,
    },
}

pub fn parse_opts() -> Opt {
    Opt::parse()
}
