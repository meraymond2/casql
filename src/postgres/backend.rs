#[derive(Debug)]
pub enum BackendMsg {
    AuthenticationCleartextPassword,
    AuthenticationMD5Password,
    AuthenticationOk,
    // BackendKeyData,
    // BindComplete,
    // Close,
    // DataRow,
    // EmptyQueryResponse,
    ErrorResponse,
    // ParameterDescription,
    ParameterStatus,
    // ParseComplete,
    ReadyForQuery,
    // RowDescription,
}

const E: u8 = 69;
const R: u8 = 82;
const S: u8 = 83;

/// Identify the message type, without parsing the entire message.
pub fn type_of(bytes: &[u8]) -> BackendMsg {
    match bytes[0] {
        E => BackendMsg::ErrorResponse,
        R => match bytes[8] {
            0 => BackendMsg::AuthenticationOk,
            3 => BackendMsg::AuthenticationCleartextPassword,
            5 => BackendMsg::AuthenticationMD5Password,
            _ => unimplemented!("R {}", bytes[8]),
        },
        S => BackendMsg::ParameterStatus,
        _ => unimplemented!("{}", bytes[0]),
    }
}
