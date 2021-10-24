// Regarding the repetition in these messages, I had a version that took a list of fields and would
// iterate over them, but it felt wasteful allocating a vector just for args. Thinking about it more,
// creating a vec of pointers is almost certainly fine. But it would also be an opportunity to
// play with macros, which would also be over-engineering, but more efficient.

/**
 * Int32 Length
 * Int16 Major Protocol Version
 * Int16 Minor Protocol Version
 * CString[]: Parameter Keys and Values
 * Null Final Byte
 *
 * For simple use cases, only the user and database parameters are relevant, so
 * I’m skipping the replication mode parameter.
 */
pub fn startup_msg(
    user: String,
    database: Option<String>,
    major_version: i16,
    minor_version: i16,
) -> Vec<u8> {
    let database_param = database.unwrap_or(user.to_owned());
    let msg_len = 4 + 2 + 2 + 5 + user.len() + 1 + 9 + database_param.len() + 1 + 1;
    let mut vec = Vec::with_capacity(1 + msg_len);
    let len_bytes = (msg_len as i32).to_be_bytes();
    vec.extend_from_slice(&len_bytes); // message length
    vec.extend_from_slice(&major_version.to_be_bytes()); // major version
    vec.extend_from_slice(&minor_version.to_be_bytes()); // minor version
    vec.extend_from_slice("user".as_bytes()); // user key
    vec.push(0x00); // user key null terminator
    vec.extend_from_slice(user.as_bytes()); // user value
    vec.push(0x00); // user value null terminator
    vec.extend_from_slice("database".as_bytes()); // database key
    vec.push(0x00); // database key null terminator
    vec.extend_from_slice(database_param.as_bytes()); // database value
    vec.push(0x00); //database database value null terminator
    vec.push(0x00); // final null terminator
    vec
}

/**
 * Int8 'p'
 * Int32 Length
 * CString hashed password
 */
pub fn password_msg(password: String) -> Vec<u8> {
    let msg_len = 4 + password.len() + 1;
    let mut vec = Vec::with_capacity(1 + msg_len);
    let len_bytes = (msg_len as i32).to_be_bytes();
    vec.push(112); // p
    vec.extend_from_slice(&len_bytes); // message length
    vec.extend_from_slice(password.as_bytes()); // password
    vec.push(0x00); // password null terminator
    vec
}

/**
* Int8 'P'
* Int32 Length
* CString Prepared Stmt Name
* CString Query String
* Int16 Number of Specified Types
* Int32[] Type Oids

 Specifying types doesn’t apply to casql, since queries will either have values passed in unescaped,
 or passed in as text. In most cases, Postgres will be able to figure them out anyway.

 It also isn’t persisting any prepared statements, so it will always use the anonymous prepared
 statement that is parsed and immediately used.
*/
pub fn parse_msg(query: &str) -> Vec<u8> {
    let msg_len = 4 + 1 + query.len() + 1 + 2;
    let types: i16 = 0;
    let mut vec = Vec::with_capacity(1 + msg_len);
    let len_bytes = (msg_len as i32).to_be_bytes();
    vec.push(0x50); // P
    vec.extend_from_slice(&len_bytes); // message length
    vec.push(0x00); // null terminator, for empty prepared statement name
    vec.extend_from_slice(query.as_bytes()); // query
    vec.push(0x00); // query null terminator
    vec.extend_from_slice(&types.to_be_bytes()); // number of types specified
    vec
}

/**
* Int8 'D'
* Int32 Length
* Int8 'S' or 'P': Stmt or Portal
* CString Name

  In casql, I’m only describing the anonymous statement that we will have just created, so the
  name will be an empty, null-terminated string.
*/
pub fn describe_msg() -> Vec<u8> {
    let msg_len = 4 + 1 + 1;
    let mut vec = Vec::with_capacity(1 + msg_len);
    let len_bytes = (msg_len as i32).to_be_bytes();
    vec.push(0x44); // D
    vec.extend_from_slice(&len_bytes); // message length
    vec.push(0x53); // S
    vec.push(0x00); // name null-terminator
    vec
}

/**
* Int8 'B'
* Int32 Length
* CString Destination Portal
* CString Prepared Statement
* Int16 Number of Param Format Codes (n)
* Int16[] n Format Codes
* Int16 Number of Param Values
* [Int32 Bytes][] Length of Param Value, Param Value
* Int16 Number of Result Format Codes (k)
* Int16[] k Format Codes

  Destination portal and statement are always anonymous. All params will be formatted as text, so
  we just specify it once. All results will be formatted as binary, so also only specified once.
*/
pub fn bind_msg(params: Vec<String>) -> Vec<u8> {
    let params_len = params.iter().fold(0, |acc, param| acc + 4 + param.len());
    let msg_len = 4 + 1 + 1 + 2 + 2 + 2 + params_len + 2 + 2;
    let mut vec = Vec::with_capacity(1 + msg_len);
    let len_bytes = (msg_len as i32).to_be_bytes();
    let param_format_len: i16 = 1;
    let param_format: i16 = 0; // 0 for text
    let param_count: i16 = params.len() as i16;
    let result_format_len: i16 = 1;
    let result_format: i16 = 1; // 1 for binary

    vec.push(0x42); // B
    vec.extend_from_slice(&len_bytes); // message length
    vec.push(0x00); // portal-name null-terminator
    vec.push(0x00); // stmt-name null-terminator
    vec.extend_from_slice(&param_format_len.to_be_bytes());
    vec.extend_from_slice(&param_format.to_be_bytes());
    vec.extend_from_slice(&param_count.to_be_bytes());
    params.iter().for_each(|param| {
        let param_len: i32 = param.len() as i32;
        vec.extend_from_slice(&param_len.to_be_bytes());
        vec.extend_from_slice(param.as_bytes());
    });
    vec.extend_from_slice(&result_format_len.to_be_bytes());
    vec.extend_from_slice(&result_format.to_be_bytes());
    vec
}

/**
* Int8 'E'
* Int32 Length
* CString Portal
* Int32 Max Rows, 0 = No Limit

  Portal is always anonymous, and we never limit the rows.
*/
pub fn execute_msg() -> Vec<u8> {
    let msg_len = 4 + 1 + 4;
    let mut vec = Vec::with_capacity(1 + msg_len);
    let len_bytes = (msg_len as i32).to_be_bytes();
    let max_rows: i32 = 0;

    vec.push(0x45); // E
    vec.extend_from_slice(&len_bytes); // message length
    vec.push(0x00); // portal-name null-terminator
    vec.extend_from_slice(&max_rows.to_be_bytes());
    vec
}

/**
 * Int8 'S'
 * Int32 Length
 */
pub fn sync_msg() -> Vec<u8> {
    let msg_len = 4;
    let mut vec = Vec::with_capacity(1 + msg_len);
    let len_bytes = (msg_len as i32).to_be_bytes();
    vec.push(0x53); // S
    vec.extend_from_slice(&len_bytes); // message length
    vec
}
