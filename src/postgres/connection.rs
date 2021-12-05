use crate::cas_err::CasErr;
use crate::args::ConnectionParams;
use crate::postgres::backend_msgs;
use crate::postgres::backend_msgs::BackendMsg;
use crate::postgres::frontend_msgs;
use crate::postgres::msg_iter::MsgIter;
use crate::postgres::json;
use crate::postgres::postgis::{POSTGIS_TYPES, POSTGIS_TYPE_QUERY};
use crate::postgres::row_iter::RowIter;
use std::collections::HashMap;
use std::io::Write;
use std::net::TcpStream;

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

        let dbname = params.database.as_ref().unwrap_or(&params.user);
        let password = params.password.as_ref().map(|pd| pd.as_str()).unwrap_or("");
        conn.send_startup(&params.user, dbname)?;
        match conn.state {
            ConnectionState::PasswordRequestedCleartext => {
                conn.send_password(password)?;
            }
            ConnectionState::PasswordRequestedMd5(salt) => {
                let hashed_pass = md5_password(&params.user, &password, salt);
                conn.send_password(&hashed_pass)?;
            }
            ConnectionState::ReadyForQuery => {}
            ConnectionState::Uninitialised => unreachable!(),
        }
        if params.postgis {
            conn.query_postgis_oids()?;
        }
        Ok(conn)
    }

    pub fn query(
        &mut self,
        query: String,
        params: Vec<String>,
    ) -> Result<(), CasErr> {
        self.stream.write(&frontend_msgs::parse_msg(&query))?;
        self.stream.write(&frontend_msgs::describe_msg())?;
        self.stream.write(&frontend_msgs::bind_msg(
            params.iter().map(|p| p.as_str()).collect(),
        ))?;
        self.stream.write(&frontend_msgs::execute_msg())?;
        self.stream.write(&frontend_msgs::sync_msg())?;
        let mut resp = MsgIter::new(&mut self.stream);
        let rows = RowIter::from(&mut resp)?;
        json::write_rows(rows, &self.dynamic_types)
    }

    fn send_startup(&mut self, user: &str, database: &str) -> Result<(), CasErr> {
        self.stream
            .write(&frontend_msgs::startup_msg(user, database, 3, 0))?;
        let mut msgs = MsgIter::new(&mut self.stream);
        if let Some(msg) = msgs.next() {
            match backend_msgs::type_of(&msg) {
                BackendMsg::AuthenticationCleartextPassword => {
                    self.state = ConnectionState::PasswordRequestedCleartext;
                    Ok(())
                }
                BackendMsg::AuthenticationMD5Password => {
                    let salt = [msg[9], msg[10], msg[11], msg[12]];
                    self.state = ConnectionState::PasswordRequestedMd5(salt);
                    Ok(())
                }
                BackendMsg::ErrorResponse => {
                    let err_msg = backend_msgs::parse_error_response(&msg);
                    Err(CasErr::PostgresErr(err_msg.to_string()))
                }
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

    fn send_password(&mut self, password: &str) -> Result<(), CasErr> {
        self.stream.write(&frontend_msgs::password_msg(password))?;
        let mut msgs = MsgIter::new(&mut self.stream);
        while let Some(msg) = msgs.next() {
            match backend_msgs::type_of(&msg) {
                BackendMsg::ErrorResponse => {
                    let err_msg = backend_msgs::parse_error_response(&msg);
                    Err(CasErr::PostgresErr(err_msg.to_string()))?;
                }
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
        self.stream
            .write(&frontend_msgs::parse_msg(&POSTGIS_TYPE_QUERY))?;
        self.stream.write(&frontend_msgs::describe_msg())?;
        self.stream.write(&frontend_msgs::bind_msg(Vec::new()))?;
        self.stream.write(&frontend_msgs::execute_msg())?;
        self.stream.write(&frontend_msgs::sync_msg())?;

        let mut resp = MsgIter::new(&mut self.stream);
        let dynamic_types = &mut self.dynamic_types;
        while let Some(msg) = resp.next() {
            match backend_msgs::type_of(&msg) {
                BackendMsg::ErrorResponse => {
                    let err_msg = backend_msgs::parse_error_response(&msg);
                    Err(CasErr::PostgresErr(err_msg.to_string()))?;
                }
                BackendMsg::ParseComplete => {}
                BackendMsg::ParameterDescription => {}
                BackendMsg::RowDescription => {}
                BackendMsg::BindComplete => {}
                BackendMsg::DataRow => {
                    let pg_type = backend_msgs::parse_type_lookup_row(&msg);
                    dynamic_types.insert(pg_type.oid, pg_type.name);
                }
                BackendMsg::Close => {}
                BackendMsg::ReadyForQuery => {
                    break;
                }
                _ => {
                    eprintln!("Received unexpected message from Postgres: {:?}", msg);
                }
            }
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
    use crate::postgres::connection::md5_password;

    #[test]
    fn test_md5() {
        assert_eq!(
            "md5ced873c22ed2ff40045eec5872ad4ea0",
            md5_password("michael", "cascat", [0x81, 0x4F, 0xA3, 0x5A])
        );
    }
}
