use crate::errors::CasErr;
use crate::query::ConnectionSpec;
use crate::casable::CasVal;
use postgres::Client;
use postgres::NoTls;
use postgres::config::Config;
use std::collections::HashMap;

pub fn exec(query: String, conn_spec: ConnectionSpec) -> Result<(), CasErr> {
    let mut conn = match conn_spec {
        ConnectionSpec::Str(url) =>
            Client::connect(&url, NoTls)?,
        ConnectionSpec::Opts(opts) => {
            let mut params = Config::new();
            params.user(&opts.user)
                .port(opts.port)
                .dbname(&opts.database)
                .host(&opts.host);
            let with_password = if opts.password.is_some() { params.password(opts.password.unwrap()) } else { &params };
            with_password.connect(NoTls)?
        }
    };


    let res = conn.query(query.as_str(), &[])?;
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
