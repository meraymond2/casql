#[derive(Debug)]
pub enum BackendMsg {
    AuthenticationCleartextPassword,
    AuthenticationMD5Password,
    AuthenticationOk,
    BackendKeyData,
    BindComplete,
    Close,
    DataRow,
    // EmptyQueryResponse,
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
