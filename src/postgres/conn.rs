use crate::postgres::backend::{type_of, BackendMsg};
use crate::postgres::frontend;
use crate::postgres::msg_iter::MsgIter;
use std::hint::unreachable_unchecked;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

pub struct ConnectionParams {
    pub host: String,
    pub user: String,
    pub password: Option<String>,
    pub database: Option<String>,
    pub port: Option<u16>,
}

pub struct Conn {
    state: ConnectionState,
    stream: TcpStream,
}

impl Conn {
    pub fn connect(params: ConnectionParams) -> Result<Self, std::io::Error> {
        let mut conn = Conn {
            state: ConnectionState::Uninitialised,
            stream: TcpStream::connect(format!("{}:{}", params.host, params.port.unwrap_or(5432)))
                .unwrap(),
        };

        conn.send_startup(params.user, params.database);
        match conn.state {
            ConnectionState::PasswordRequestedCleartext => {
                conn.send_password(params.password.unwrap_or(String::from("")));
            }
            ConnectionState::PasswordRequestedMd5 => {
                conn.send_password(params.password.unwrap_or(String::from("")));
            }
            ConnectionState::ReadyForQuery => {}
            ConnectionState::Uninitialised => unreachable!(),
        }
        Ok(conn)
    }

    fn send_startup(&mut self, user: String, database: Option<String>) {
        self.stream
            .write(&frontend::startup_msg(user, database, 3, 0))
            .unwrap();
        let mut msgs = MsgIter::new(&mut self.stream);
        while let Some(msg) = msgs.next() {
            match type_of(&msg) {
                BackendMsg::AuthenticationCleartextPassword => {
                    self.state = ConnectionState::PasswordRequestedCleartext;
                    break;
                }
                BackendMsg::AuthenticationMD5Password => {
                    self.state = ConnectionState::PasswordRequestedMd5;
                    break;
                }
                BackendMsg::AuthenticationOk => {}
                BackendMsg::ErrorResponse => {
                    todo!("handle postgres errors");
                }
                BackendMsg::ReadyForQuery => {
                    self.state = ConnectionState::ReadyForQuery;
                    break;
                }
            }
        }
    }

    fn send_password(&mut self, password: String) {
        self.stream
            .write(&frontend::password_msg(password))
            .unwrap();
        let mut msgs = MsgIter::new(&mut self.stream);
        while let Some(msg) = msgs.next() {
            match type_of(&msg) {
                BackendMsg::ErrorResponse => {
                    todo!("handle postgres errors");
                }
                BackendMsg::ReadyForQuery => {
                    self.state = ConnectionState::ReadyForQuery;
                    break;
                }
                _ => {
                    println!("{:?}", msg);
                }
            }
        }
    }
}

#[derive(Debug)]
enum ConnectionState {
    PasswordRequestedCleartext,
    PasswordRequestedMd5,
    ReadyForQuery,
    Uninitialised,
}
