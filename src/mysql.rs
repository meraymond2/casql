use crate::errors::CasErr;
use crate::query::ConnectionSpec;
use mysql::{Conn, Opts, OptsBuilder};

pub fn exec(query: String, conn_spec: ConnectionSpec) -> Result<(), CasErr> {
  let mut conn = match conn_spec {
    ConnectionSpec::Str(url) => {
      let conn_url = Opts::from_url(&url)?;
      Conn::new(conn_url)
    }
    ConnectionSpec::Opts(opts) => {
      let mut builder = OptsBuilder::new();
      builder
        .ip_or_hostname(Some(opts.host))
        .tcp_port(opts.port)
        .user(Some(opts.user))
        .pass(opts.password)
        .db_name(Some(opts.database));
      Conn::new(builder)
    }
  }?;
  let res = conn.query(query)?;
  let columns = res.column_indexes();
  for row_opt in res {
    let mut row = row_opt?;
    println!("{:?}", row);
    for val in columns.values() {
      let v: mysql::Value = row.take(*val).unwrap();
      println!("Column: {:?}, Row: {:?}.", val, v)
    }
    // println!("{:?}", row.take())
  }
  Ok(())
}
