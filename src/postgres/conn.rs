use crate::postgres::frontend;
use crate::postgres::msg_iter::MsgIter;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;
use crate::postgres::backend::{deserialise, BackendMsg};

pub struct ConnectionParams {
    pub host: String,
    pub user: String,
    pub password: Option<String>,
    pub database: Option<String>,
    pub port: Option<u16>,
}

pub struct Conn {
    stream: TcpStream,
}

impl Conn {
    pub fn connect(params: ConnectionParams) -> Result<Self, std::io::Error> {
        let mut stream =
            TcpStream::connect(format!("{}:{}", params.host, params.port.unwrap_or(5432)))?;
        stream
            .set_read_timeout(Some(Duration::from_millis(1)))
            .unwrap();
        stream.write(frontend::startup_msg(params.user, params.database, 3, 0).as_slice())?;
        let msgs = MsgIter::new(&mut stream);
        Ok(Conn { stream })
    }
}
