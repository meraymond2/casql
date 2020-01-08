use crate::casable::CasVal;
use crate::errors::CasErr;
use crate::query::ConnectionSpec;
use mysql::{Conn, Opts, OptsBuilder};

// TODOS:
// 1. Implement From Value -> CasVal
// 2. Handle the various mysql errors
// 3. Implement the json printing

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
  let mut rows: Vec<std::collections::HashMap<String, CasVal>> = Vec::new();
  let columns = res.column_indexes();

  for tuple_opt in res {
    let mut tuple = tuple_opt?;
    let mut row: std::collections::HashMap<String, CasVal> =
      std::collections::HashMap::new();

    for (name, idx) in &columns {
      let my_v: mysql::Value = tuple.take(*idx).unwrap();
      let cas_v: CasVal = my_v.into();
      row.insert(name.to_owned(), cas_v);
    }
    rows.push(row);
  }

  let json = serde_json::to_string(&rows).unwrap();
  println!("{}", json);
  Ok(())
}
