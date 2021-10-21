use crate::cas_err::CasErr;
use crate::postgres::backend;
use crate::postgres::backend::BackendMsg;
use crate::postgres::frontend;
use crate::postgres::json_writer::write_json_rows;
use crate::postgres::msg_iter::MsgIter;
use crate::postgres::postgis::{parse_type_lookup, POSTGIS_QUERY, POSTGIS_TYPES};
use std::collections::HashMap;
use std::convert::TryInto;
use std::io::Write;
use std::net::TcpStream;

pub struct ConnectionParams {
    pub host: String,
    pub user: String,
    pub password: Option<String>,
    pub database: Option<String>,
    pub port: Option<u16>,
    pub postgis: bool,
}

#[derive(Debug)]
pub struct Conn {
    state: ConnectionState,
    stream: TcpStream,
    dynamic_types: HashMap<i32, String>,
}

impl Conn {
    pub fn connect(params: ConnectionParams) -> Result<Self, CasErr> {
        let dynamic_types = if params.postgis {
            HashMap::with_capacity(POSTGIS_TYPES.len())
        } else {
            HashMap::new()
        };
        let mut conn = Conn {
            state: ConnectionState::Uninitialised,
            stream: TcpStream::connect(format!("{}:{}", params.host, params.port.unwrap_or(5432)))?,
            dynamic_types,
        };

        conn.send_startup(params.user.clone(), params.database)?;
        match conn.state {
            ConnectionState::PasswordRequestedCleartext => {
                conn.send_password(params.password.unwrap_or(String::from("")))?;
            }
            ConnectionState::PasswordRequestedMd5(salt) => {
                let password = params.password.unwrap_or(String::from(""));
                let hashed_pass = md5_password(&params.user, &password, salt);
                conn.send_password(hashed_pass)?;
            }
            ConnectionState::ReadyForQuery => {}
            ConnectionState::Uninitialised => unreachable!(),
        }
        if params.postgis {
            conn.query_postgis_oids()?;
        }
        Ok(conn)
    }

    pub fn query(&mut self, query: String, params: Vec<String>) -> Result<(), CasErr> {
        self.stream.write(&frontend::parse_msg(&query))?;
        self.stream.write(&frontend::describe_msg())?;
        self.stream.write(&frontend::bind_msg(params))?;
        self.stream.write(&frontend::execute_msg())?;
        self.stream.write(&frontend::sync_msg())?;
        let mut resp = MsgIter::new(&mut self.stream);
        write_json_rows(&mut resp, &self.dynamic_types)
    }

    fn send_startup(&mut self, user: String, database: Option<String>) -> Result<(), CasErr> {
        self.stream
            .write(&frontend::startup_msg(user, database, 3, 0))?;
        let mut msgs = MsgIter::new(&mut self.stream);
        if let Some(msg) = msgs.next() {
            match backend::type_of(&msg) {
                BackendMsg::AuthenticationCleartextPassword => {
                    self.state = ConnectionState::PasswordRequestedCleartext;
                    Ok(())
                }
                BackendMsg::AuthenticationMD5Password => {
                    let salt: [u8; 4] = msg[9..13]
                        .try_into()
                        .expect("Slice should be right length.");
                    self.state = ConnectionState::PasswordRequestedMd5(salt);
                    Ok(())
                }
                // BackendMsg::ErrorResponse => {
                //     todo!("handle postgres errors");
                // }
                BackendMsg::ReadyForQuery => {
                    self.state = ConnectionState::ReadyForQuery;
                    Ok(())
                }
                _ => Err(CasErr::PostgresErr(format!(
                    "Received unexpected message from Postgres: {:?}",
                    msg
                ))),
            }
        } else {
            Err(CasErr::PostgresErr(
                "Expected response, but didn’t receive any.".to_owned(),
            ))
        }
    }

    fn send_password(&mut self, password: String) -> Result<(), CasErr> {
        self.stream.write(&frontend::password_msg(password))?;
        let mut msgs = MsgIter::new(&mut self.stream);
        while let Some(msg) = msgs.next() {
            match backend::type_of(&msg) {
                // BackendMsg::ErrorResponse => {
                //     todo!("handle postgres errors");
                // }
                BackendMsg::AuthenticationOk => {}
                BackendMsg::ParameterStatus => {}
                BackendMsg::BackendKeyData => {}
                BackendMsg::ReadyForQuery => {
                    self.state = ConnectionState::ReadyForQuery;
                    break;
                }
                _ => Err(CasErr::PostgresErr(format!(
                    "Received unexpected message from Postgres: {:?}",
                    msg
                )))?,
            }
        }
        Ok(())
    }

    fn query_postgis_oids(&mut self) -> Result<(), CasErr> {
        self.stream.write(&frontend::parse_msg(&POSTGIS_QUERY))?;
        self.stream.write(&frontend::describe_msg())?;
        self.stream.write(&frontend::bind_msg(Vec::new()))?;
        self.stream.write(&frontend::execute_msg())?;
        self.stream.write(&frontend::sync_msg())?;
        let mut resp = MsgIter::new(&mut self.stream);
        let mut pg_types = parse_type_lookup(&mut resp);
        while let Some(pg_type) = pg_types.pop() {
            self.dynamic_types.insert(pg_type.oid, pg_type.name);
        }
        Ok(())
    }
}

#[derive(Debug)]
enum ConnectionState {
    PasswordRequestedCleartext,
    PasswordRequestedMd5([u8; 4]),
    ReadyForQuery,
    Uninitialised,
}

// Directly borrowed from rust-postgres (https://github.com/sfackler/rust-postgres).
fn md5_password(user: &str, password: &str, salt: [u8; 4]) -> String {
    let mut context = md5::Context::new();
    context.consume(password);
    context.consume(user);
    let output = context.compute();
    context = md5::Context::new();
    context.consume(format!("{:x}", output));
    context.consume(&salt);
    format!("md5{:x}", context.compute())
}

#[cfg(test)]
mod tests {
    use crate::postgres::conn::md5_password;

    #[test]
    fn test_md5() {
        assert_eq!(
            "md5ced873c22ed2ff40045eec5872ad4ea0",
            md5_password("michael", "cascat", [0x81, 0x4F, 0xA3, 0x5A])
        );
    }
}
