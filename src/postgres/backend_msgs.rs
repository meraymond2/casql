use crate::binary_reader::{BinaryReader, ByteOrder};
use crate::postgres::row_iter::CasVal;
use crate::postgres::types;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

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
    ErrorResponse,
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
        0x45 => BackendMsg::ErrorResponse,
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
//
// #[derive(Debug)]
// pub struct Field {
//     pub name: String,
//     pub data_type_oid: i32,
// }
//
// /**
//  * Int8 'T'
//  * Int32 Length
//  * Int16 Number of Fields
//  *
//  * String Field Name
//  * Int32 Table OID
//  * Int16 Column #
//  * Int32 Data Type OID
//  * Int16 Data Type Size
//  * Int32 Type Modifier
//  * Int16 Format Code
//  */
// pub fn parse_row_desc(bytes: Vec<u8>) -> Vec<Field> {
//     let mut msg = BinaryMsg::from(bytes);
//     // skip discriminator and message size
//     msg.skip(5);
//     let field_count = msg.i16();
//     let mut fields = Vec::with_capacity(field_count as usize);
//
//     for _ in 0..field_count {
//         let name = msg.c_str();
//         // skip table_oid (i32) and column (i16)
//         msg.skip(6);
//         let data_type_oid = msg.i32();
//         // skip data_type_size (i16), type_modifier (i32) and format_code (i16)
//         msg.skip(8);
//         fields.push({
//             Field {
//                 name,
//                 data_type_oid,
//             }
//         })
//     }
//     fields
// }

// TODO: There are more fields that may or may not present, that I could add in later, and include
// in the message if they exist.
pub struct ErrorResponse {
    code: String,
    severity: String,
    message: String,
}

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} ({})", self.severity, self.message, self.code)
    }
}

/**
 * Int8 'E'
 * Int32 Length
 * Int8 Error Field
 *
 * String Field Value
 * Null Terminator
 */
pub fn parse_error_response(bytes: &[u8]) -> ErrorResponse {
    let mut rdr = BinaryReader::from(bytes, ByteOrder::BigEndian);
    // skip tag and message length
    rdr.skip(5);

    let mut fields = HashMap::new();
    loop {
        let error_key = rdr.u8();
        if error_key == 0x00 {
            break;
        }
        let error_msg = rdr.c_str();
        fields.insert(error_key as char, error_msg);
    }
    ErrorResponse {
        code: fields.remove(&'C').unwrap_or("".to_string()),
        severity: fields.remove(&'S').unwrap_or("".to_string()),
        message: fields.remove(&'M').unwrap_or("".to_string()),
    }
}

#[derive(Clone, Debug)]
pub struct Field {
    pub name: String,
    pub data_type_oid: i32,
}

/**
 * Int8 'T'
 * Int32 Length
 * Int16 Number of Fields
 *
 * String Field Name
 * Int32 Table OID
 * Int16 Column #
 * Int32 Data Type OID
 * Int16 Data Type Size
 * Int32 Type Modifier
 * Int16 Format Code
 */
pub fn parse_row_desc(bytes: &[u8]) -> Vec<Field> {
    let mut rdr = BinaryReader::from(bytes, ByteOrder::BigEndian);
    // skip discriminator and message size
    rdr.skip(5);
    let field_count = rdr.i16();
    let mut fields = Vec::with_capacity(field_count as usize);

    for _ in 0..field_count {
        let name = rdr.c_str();
        // skip table_oid (i32) and column (i16)
        rdr.skip(6);
        let data_type_oid = rdr.i32();
        // skip data_type_size (i16), type_modifier (i32) and format_code (i16)
        rdr.skip(8);
        fields.push({
            Field {
                name,
                data_type_oid,
            }
        })
    }
    fields
}

/**
 * Int8 'D'
 * Int32 Length
 * Int16 Number of Values
 *
 * Int32 Value Length (NULL is -1)
 * Bytes Column Value
 */
pub fn parse_data_row(
    msg: &[u8],
    parse: &mut impl FnMut(Option<&[u8]>, usize) -> (String, CasVal),
) -> HashMap<String, CasVal> {
    let mut rdr = BinaryReader::from(&msg, ByteOrder::BigEndian);
    // skip discriminator, message size
    rdr.skip(5);

    let mut parsed = HashMap::new();
    let value_count = rdr.i16() as usize;
    for idx in 0..value_count {
        let value_len = rdr.i32();
        let value_bytes = if value_len == -1 {
            None
        } else {
            Some(rdr.byte_slice(value_len as usize))
        };
        let (name, value) = parse(value_bytes, idx);
        parsed.insert(name, value);
    }
    parsed
}

#[derive(Debug)]
pub struct PgType {
    pub name: String,
    pub oid: i32,
}

/**
 * Int8 'D'
 * Int32 Length
 * Int16 Number of Values
 *
 * Int32 Value Length (NULL is -1)
 * Bytes Column Value
 *
 * This specific function will be a subset of DataRow, where the query was
 * SELECT typname, oid FROM pg_types WHERE...
 * so we can assume the result types.
 */
pub fn parse_type_lookup_row(msg: &[u8]) -> PgType {
    let mut rdr = BinaryReader::from(msg, ByteOrder::BigEndian);
    // skip discriminator, message size and value count (which is always 2)
    rdr.skip(7);

    let name_len = rdr.i32();
    let name_bytes = rdr.byte_slice(name_len as usize);
    let name = std::str::from_utf8(&name_bytes)
        .expect("Value will be a valid UTF-8 string.")
        .to_owned();
    rdr.skip(4); // skip oid length, it’s always 4
    let oid = rdr.i32();

    PgType { name, oid }
}
