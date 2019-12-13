use crate::casable::CasableValue;
use crate::model::ConnectionSpec;
use postgres::params::{ConnectParams, Host};
use postgres::{Connection, TlsMode};

use std::ops::Deref;

// for mapping Option<String> to Option<&str>, I need to figure out long
// term what the proper lifecycles are, and then I don't need to pass this
// function owned Strings, since it doesn't want them.
trait OptionDeref<T: Deref> {
  fn as_deref(&self) -> Option<&T::Target>;
}

impl<T: Deref> OptionDeref<T> for Option<T> {
  fn as_deref(&self) -> Option<&T::Target> {
    self.as_ref().map(Deref::deref)
  }
}

pub fn run_query(query: String, conn_spec: ConnectionSpec) {
  let conn = match conn_spec {
    ConnectionSpec::Opts(conn_opts) => {
      let params = ConnectParams::builder()
        .user(&conn_opts.user, conn_opts.password.as_deref())
        .port(conn_opts.port)
        .database(&conn_opts.database)
        .build(Host::Tcp(conn_opts.host));
      Connection::connect(params, TlsMode::None).unwrap()
    }
    ConnectionSpec::Str(conn_string) => Connection::connect(conn_string, TlsMode::None).unwrap(),
  };

  let res = conn.query(&query, &[]).unwrap();

  let mut returns: Vec<std::collections::HashMap<String, CasableValue>> = Vec::new();
  for row in res.into_iter() {
    let count = row.len();
    let names: Vec<String> = row
      .columns()
      .into_iter()
      .map(|col| String::from(col.name()))
      .collect();
    let mut thing: std::collections::HashMap<String, CasableValue> =
      std::collections::HashMap::new();
    for i in 0..count {
      let x: CasableValue = row.get(i);
      thing.insert(names.get(i).unwrap().clone(), x);
    }
    returns.push(thing.clone());
  }
  let y = serde_json::to_string(&returns).unwrap();
  println!("{0}", y);
  // println!("Map: {:?}", thing);
}

// https://github.com/sfackler/rust-postgres/issues/21
