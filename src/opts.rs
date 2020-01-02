
use clap::Clap;

#[derive(Clap, Debug)]
pub enum Connection {
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
   Connection(Connection)
}

// let list = SubCommand::with_name("list").about("List saved connections");

// let conn = Arg::with_name(CONN).help("The name of a connection");
// let describe = SubCommand::with_name("describe")
//   .about("Describe a saved connection")
//   .arg(Arg::from(&conn).required(true));
