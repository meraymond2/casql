pub enum BackendMsg {
    AuthenticationCleartextPassword,
    AuthenticationMD5Password([u8; 4]),
    AuthenticationOk,
    // BackendKeyData,
    // BindComplete,
    // Close,
    // DataRow,
    // EmptyQueryResponse,
    // ErrorResponse,
    // ParameterDescription,
    // ParameterStatus,
    // ParseComplete,
    // ReadyForQuery,
    // RowDescription,
}

const R: u8 = 82;

pub fn deserialise(bytes: &[u8]) -> BackendMsg {
    match bytes[0] {
        R => match bytes[8] {
            0 => BackendMsg::AuthenticationOk,
            3 => BackendMsg::AuthenticationCleartextPassword,
            5 => {
                let mut salt: [u8; 4] = [0; 4];
                salt.copy_from_slice(&bytes[9..13]);
                BackendMsg::AuthenticationMD5Password(salt)
            },
            _ => unimplemented!("aaah"),
        },
        other => unimplemented!("aaaaah {}", other),
    }
}
