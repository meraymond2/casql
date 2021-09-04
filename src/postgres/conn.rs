use crate::postgres::frontend;
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
        msgs.for_each(|msg| println!("! {:?}", msg));
        Ok(Conn { stream })
    }
}

struct MsgIter<'stream> {
    stream: &'stream mut TcpStream,
    buf: [u8; 5],
    len: usize,
    pos: usize,
}

impl<'stream> MsgIter<'stream> {
    pub fn new(stream: &'stream mut TcpStream) -> Self {
        MsgIter {
            stream,
            buf: [0; 5],
            len: 0,
            pos: 0,
        }
    }

    fn read_bytes(&mut self) -> Result<(), std::io::Error> {
        match self.stream.read(&mut self.buf) {
            Ok(bytes_read) => {
                self.len = bytes_read;
                self.pos = 0;
                Ok(())
            }
            Err(error) => match error.kind() {
                // We want to read until Postgres has stopped sending data, but cannot rely on the
                // message length alone, because I can’t predict how many messages there will be.
                std::io::ErrorKind::WouldBlock => {
                    self.len = 0;
                    self.pos = 0;
                    Ok(())
                }
                _ => Err(error),
            },
        }
    }
}

impl<'stream> Iterator for MsgIter<'stream> {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        // 1. We either haven’t started, or finished the previous message exactly at the end of the
        // buffer. If there is no more to read, return None, else read more bytes and start over.
        if self.pos == self.len {
            // read
            self.read_bytes().unwrap();
            if self.len > 0 {
                self.next()
            } else {
                None
            }
        } else if self.len - self.pos > 5 {
            // 2. There are bytes to read, and at least enough to get the message length.
            let mut len_bytes: [u8; 4] = [0; 4];
            len_bytes.copy_from_slice(&self.buf[(self.pos + 1)..self.pos + 5]);
            let msg_len = i32::from_be_bytes(len_bytes); // can I simplify this to one line? // technically i32
            let mut to_copy = 1 + msg_len as usize;
            let mut msg = Vec::with_capacity(to_copy);
            while to_copy > 0 {
                if to_copy <= self.len - self.pos {
                    msg.extend_from_slice(&self.buf[self.pos..to_copy]);
                    self.pos = self.pos + to_copy;
                    to_copy = 0;
                } else {
                    msg.extend_from_slice(&self.buf[self.pos..self.len]);
                    to_copy = to_copy - (self.len - self.pos);
                    self.read_bytes();
                }
            }
            Some(msg)
        } else {
            // 3. There are bytes to read, but not enough left in the buffer to determine the message length.
            let msg_type: u8 = self.buf[self.pos];
            self.pos = self.pos + 1;
            let mut len_bytes: [u8; 4] = [0; 4];
            self.buf[self.pos..self.len]
                .iter()
                .enumerate()
                .for_each(|(idx, byte)| len_bytes[idx] = *byte);
            let copied = self.len - self.pos;
            self.read_bytes();
            self.buf[self.pos..(4 - copied)]
                .iter()
                .enumerate()
                .for_each(|(idx, byte)| len_bytes[copied + idx] = *byte);
            let msg_len = i32::from_be_bytes(len_bytes);


            let mut to_copy = (msg_len - 4) as usize;
            let mut msg = Vec::with_capacity(5 + to_copy);
            while to_copy > 0 {
                if to_copy <= self.len - self.pos {
                    msg.extend_from_slice(&self.buf[self.pos..to_copy]);
                    self.pos = self.pos + to_copy;
                    to_copy = 0;
                } else {
                    msg.extend_from_slice(&self.buf[self.pos..self.len]);
                    to_copy = to_copy - (self.len - self.pos);
                    self.read_bytes();
                }
            }
            Some(msg)
        }
    }
}
