use crate::postgres::frontend;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};

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
        stream.write(frontend::startup_msg(params.user, params.database, 3, 0).as_slice())?;
        let mut buf = [0; 256];
        let bytes_read = stream.read(&mut buf)?;
        println!("Read: {}, {:?}", bytes_read, buf);
        Ok(Conn { stream })
    }
}
