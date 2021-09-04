use std::io::Read;
use std::net::TcpStream;
use std::time::Duration;

const BUFFER_SIZE: usize = 5;

pub struct MsgIter<'stream> {
    stream: &'stream mut TcpStream,
    buf: [u8; BUFFER_SIZE],
    len: usize,
    pos: usize,
}

impl<'stream> MsgIter<'stream> {
    pub fn new(stream: &'stream mut TcpStream) -> Self {
        MsgIter {
            stream,
            buf: [0; BUFFER_SIZE],
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

    fn copy_msg_bytes(&mut self, msg: &mut Vec<u8>, bytes_to_copy: usize) {
        let mut n = bytes_to_copy;
        while n > 0 {
            if n <= self.len - self.pos {
                msg.extend_from_slice(&self.buf[self.pos..n]);
                self.pos = self.pos + n;
                n = 0;
            } else {
                msg.extend_from_slice(&self.buf[self.pos..self.len]);
                n = n - (self.len - self.pos);
                self.read_bytes().unwrap();
            }
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
            // 2. There are bytes to read, and at least enough in the buffer to get the message length.
            let mut len_bytes: [u8; 4] = [0; 4];
            len_bytes.copy_from_slice(&self.buf[(self.pos + 1)..self.pos + 5]);
            let msg_len = i32::from_be_bytes(len_bytes); // can I simplify this to one line? // technically i32
            let to_copy = 1 + msg_len as usize;
            let mut msg = Vec::with_capacity(to_copy);
            self.copy_msg_bytes(&mut msg, to_copy);
            Some(msg)
        } else {
            // 3. There are bytes to read, but not enough left in the buffer to determine the message length.
            // We need to keep the bytes that are there, read more into the buffer, then finish getting
            // the length, and read the rest of the message.
            let msg_type: u8 = self.buf[self.pos];
            self.pos = self.pos + 1;
            let mut len_bytes: [u8; 4] = [0; 4];
            self.buf[self.pos..self.len]
                .iter()
                .enumerate()
                .for_each(|(idx, byte)| len_bytes[idx] = *byte);
            let copied = self.len - self.pos;
            self.read_bytes().unwrap();
            self.buf[self.pos..(4 - copied)]
                .iter()
                .enumerate()
                .for_each(|(idx, byte)| len_bytes[copied + idx] = *byte);
            let msg_len = i32::from_be_bytes(len_bytes) as usize;

            let mut msg = Vec::with_capacity(msg_len + 1);
            msg.push(msg_type);
            msg.extend_from_slice(&len_bytes);
            self.copy_msg_bytes(&mut msg, msg_len - 4);
            Some(msg)
        }
    }
}
