use crate::errors::CasErr;
use crate::query::ConnectionSpec;
use crate::casable::CasVal;
use postgres::params::{ConnectParams, Host};
use postgres::{Connection, TlsMode};
use std::collections::HashMap;

pub fn exec(query: String, conn_spec: ConnectionSpec) -> Result<(), CasErr> {
  let conn = match conn_spec {
    ConnectionSpec::Str(url) =>
      Connection::connect(url, TlsMode::None)?,
    ConnectionSpec::Opts(opts) => {
      let params = ConnectParams::builder()
        .user(&opts.user, opts.password.as_deref())
        .port(opts.port)
        .database(&opts.database)
        .build(Host::Tcp(opts.host));
      Connection::connect(params, TlsMode::None)?
    }
  };


  let res = conn.query(&query, &[])?;
  let mut rows: Vec<HashMap<String, CasVal>> = Vec::new();

  // sub-optimal, will refactor later
  for row in res.into_iter() {
    let count = row.len();
    let names: Vec<String> = row
      .columns()
      .iter()
      .map(|col| String::from(col.name()))
      .collect();
    let mut record: HashMap<String, CasVal> =
      std::collections::HashMap::new();
    for i in 0..count {
      let val: CasVal = row.get(i);
      record.insert(names.get(i).expect("unreachable").clone(), val);
    }
    rows.push(record);
  }
  let json = serde_json::to_string(&rows).unwrap(); // TODO
  println!("{}", json);
  Ok(())
}
