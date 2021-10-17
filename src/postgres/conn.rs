use crate::postgres::backend;
use crate::postgres::backend::BackendMsg;
use crate::postgres::frontend;
use crate::postgres::json_writer::write_json_rows;
use crate::postgres::msg_iter::MsgIter;
use std::convert::TryInto;
use std::io::Write;
use std::net::TcpStream;

pub struct ConnectionParams {
    pub host: String,
    pub user: String,
    pub password: Option<String>,
    pub database: Option<String>,
    pub port: Option<u16>,
}

#[derive(Debug)]
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

        conn.send_startup(params.user.clone(), params.database);
        match conn.state {
            ConnectionState::PasswordRequestedCleartext => {
                conn.send_password(params.password.unwrap_or(String::from("")));
            }
            ConnectionState::PasswordRequestedMd5(salt) => {
                let password = params.password.unwrap_or(String::from(""));
                let hashed_pass = md5_password(&params.user, &password, salt);
                conn.send_password(hashed_pass);
            }
            ConnectionState::ReadyForQuery => {}
            ConnectionState::Uninitialised => unreachable!(),
        }
        Ok(conn)
    }

    pub fn query(&mut self, query: String, params: Vec<String>) {
        self.stream.write(&frontend::parse_msg(&query)).unwrap();
        self.stream.write(&frontend::describe_msg()).unwrap();
        self.stream.write(&frontend::bind_msg(params)).unwrap();
        self.stream.write(&frontend::execute_msg()).unwrap();
        self.stream.write(&frontend::sync_msg()).unwrap();
        let mut msgs = MsgIter::new(&mut self.stream);
        write_json_rows(&mut msgs);
    }

    fn send_startup(&mut self, user: String, database: Option<String>) {
        self.stream
            .write(&frontend::startup_msg(user, database, 3, 0))
            .unwrap();
        let mut msgs = MsgIter::new(&mut self.stream);
        while let Some(msg) = msgs.next() {
            match backend::type_of(&msg) {
                BackendMsg::AuthenticationCleartextPassword => {
                    self.state = ConnectionState::PasswordRequestedCleartext;
                    break;
                }
                BackendMsg::AuthenticationMD5Password => {
                    let salt: [u8; 4] = msg[9..13].try_into().unwrap();
                    self.state = ConnectionState::PasswordRequestedMd5(salt);
                    break;
                }
                BackendMsg::ErrorResponse => {
                    todo!("handle postgres errors");
                }
                BackendMsg::ReadyForQuery => {
                    self.state = ConnectionState::ReadyForQuery;
                    break;
                }
                _ => {}
            }
        }
    }

    fn send_password(&mut self, password: String) {
        self.stream
            .write(&frontend::password_msg(password))
            .unwrap();
        let mut msgs = MsgIter::new(&mut self.stream);
        while let Some(msg) = msgs.next() {
            match backend::type_of(&msg) {
                BackendMsg::ErrorResponse => {
                    todo!("handle postgres errors");
                }
                BackendMsg::ReadyForQuery => {
                    self.state = ConnectionState::ReadyForQuery;
                    break;
                }
                _ => {}
            }
        }
    }
}

#[derive(Debug)]
enum ConnectionState {
    PasswordRequestedCleartext,
    PasswordRequestedMd5([u8; 4]),
    ReadyForQuery,
    Uninitialised,
}

// Cheerfully stolen from rust-postgres (https://github.com/sfackler/rust-postgres).
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
