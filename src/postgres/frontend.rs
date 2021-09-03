pub fn startup_msg(
    user: String,
    database: Option<String>,
    major_version: i16,
    minor_version: i16,
) -> Vec<u8> {
    serialise_msg(
        None,
        vec![
            MsgField::Int16(major_version),
            MsgField::Int16(minor_version),
            MsgField::CStr("user".to_owned()),
            MsgField::CStr(user.to_owned()),
            MsgField::CStr("database".to_owned()),
            MsgField::CStr(database.unwrap_or(user.to_owned())),
            MsgField::Int8(0x00),
        ],
    )
}

enum MsgField {
    Int8(i8),
    Int16(i16),
    Int32(i32),
    CStr(String),
    Str(String),
}

impl MsgField {
    pub fn len(&self) -> i32 {
        match self {
            MsgField::Int8(_) => 1,
            MsgField::Int16(_) => 2,
            MsgField::Int32(_) => 4,
            MsgField::CStr(s) => s.len() as i32 + 1,
            MsgField::Str(s) => s.len() as i32,
        }
    }
}

fn serialise_msg(msg_type: Option<u8>, fields: Vec<MsgField>) -> Vec<u8> {
    let msg_len = 4 + fields.iter().fold(0, |len, field| len + field.len());
    let total_len = msg_type.map_or(0, |_| 1) + msg_len;
    let mut bytes = Vec::with_capacity(total_len as usize);
    if let Some(msg_id) = msg_type {
        bytes.push(msg_id);
    }
    bytes.extend_from_slice(&msg_len.to_be_bytes());
    fields.iter().fold(bytes, |mut acc, field| match field {
        MsgField::Int8(i) => {
            acc.extend_from_slice(&i.to_be_bytes());
            acc
        }
        MsgField::Int16(i) => {
            acc.extend_from_slice(&i.to_be_bytes());
            acc
        }
        MsgField::Int32(i) => {
            acc.extend_from_slice(&i.to_be_bytes());
            acc
        }
        MsgField::CStr(s) => {
            acc.extend_from_slice(&s.as_bytes());
            acc.push(0x00);
            acc
        }
        MsgField::Str(s) => {
            acc.extend_from_slice(&s.as_bytes());
            acc
        }
    })
}
