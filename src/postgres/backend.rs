#[derive(Debug)]
pub enum BackendMsg {
    AuthenticationCleartextPassword,
    AuthenticationMD5Password,
    AuthenticationOk,
    BackendKeyData,
    BindComplete,
    Close,
    DataRow,
    // EmptyQueryResponse, // TODO
    // ErrorResponse, // TODO
    ParameterDescription,
    ParameterStatus,
    ParseComplete,
    ReadyForQuery,
    RowDescription,
}

/// Identify the message type, without parsing the entire message.
pub fn type_of(bytes: &[u8]) -> BackendMsg {
    match bytes[0] {
        0x31 => BackendMsg::ParseComplete,
        0x32 => BackendMsg::BindComplete,
        0x43 => BackendMsg::Close,
        0x44 => BackendMsg::DataRow,
        // 0x45 => BackendMsg::ErrorResponse,
        0x4B => BackendMsg::BackendKeyData,
        0x52 => match bytes[8] {
            0x00 => BackendMsg::AuthenticationOk,
            0x03 => BackendMsg::AuthenticationCleartextPassword,
            0x05 => BackendMsg::AuthenticationMD5Password,
            _ => unimplemented!("R {}", bytes[8]),
        },
        0x53 => BackendMsg::ParameterStatus,
        0x54 => BackendMsg::RowDescription,
        0x5A => BackendMsg::ReadyForQuery,
        0x74 => BackendMsg::ParameterDescription,
        _ => unimplemented!("{}", bytes[0]),
    }
}

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub data_type_oid: i32,
}

pub fn parse_row_desc(bytes: Vec<u8>) -> Vec<Field> {
    let mut msg = BinaryMsg::from(bytes);
    // skip discriminator and message size
    msg.skip(5);
    let field_count = msg.i16();
    let mut fields = Vec::with_capacity(field_count as usize);

    for _ in 0..field_count {
        let name = msg.c_str();
        // skip table_oid (i32) and column (i16)
        msg.skip(6);
        let data_type_oid = msg.i32();
        // skip data_type_size (i16), type_modifier (i32) and format_code (i16)
        msg.skip(8);
        fields.push({
            Field {
                name,
                data_type_oid,
            }
        })
    }
    fields
}

pub struct BinaryMsg {
    bytes: Vec<u8>,
    pos: usize,
}

impl BinaryMsg {
    pub fn from(bytes: Vec<u8>) -> Self {
        BinaryMsg { bytes, pos: 0 }
    }

    pub fn skip(&mut self, n: usize) {
        self.pos = self.pos + n;
    }

    pub fn i16(&mut self) -> i16 {
        let mut byte_arr: [u8; 2] = [0; 2];
        byte_arr.copy_from_slice(&self.bytes[(self.pos)..(self.pos + 2)]);
        let n = i16::from_be_bytes(byte_arr);
        self.pos += 2;
        n
    }

    pub fn i32(&mut self) -> i32 {
        let mut byte_arr: [u8; 4] = [0; 4];
        byte_arr.copy_from_slice(&self.bytes[(self.pos)..(self.pos + 4)]);
        let n = i32::from_be_bytes(byte_arr);
        self.pos += 4;
        n
    }

    pub fn c_str(&mut self) -> String {
        let start = self.pos;
        while self.bytes[self.pos] != 0x00 {
            self.pos += 1
        }
        let s = std::str::from_utf8(&self.bytes[start..self.pos])
            .expect("Value will be a valid UTF-8 string.")
            .to_owned();
        self.skip(1); // skip the null terminator
        s
    }

    pub fn bytes(&mut self, len: usize) -> Vec<u8> {
        let slice = &self.bytes[self.pos..(self.pos + len)];
        let vec = slice.to_vec();
        self.skip(len);
        vec
    }
}
