use crate::postgres::frontend_msgs::Element::CStr;

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
pub fn startup_msg(user: &str, database: &str, major_version: i16, minor_version: i16) -> Vec<u8> {
    build_msg(
        None,
        vec![
            Element::Int16(major_version),
            Element::Int16(minor_version),
            Element::CStr("user"),
            Element::CStr(user),
            Element::CStr("database"),
            Element::CStr(database),
            Element::Byte(0),
        ],
    )
}

/**
 * Int8 'p'
 * Int32 Length
 * CString hashed password
 */
pub fn password_msg(password: &str) -> Vec<u8> {
    build_msg(Some('p'), vec![Element::CStr(password)])
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
    build_msg(
        Some('P'),
        vec![Element::CStr(""), Element::CStr(query), Element::Int16(0)],
    )
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
    build_msg(Some('d'), vec![Element::Byte('S' as u8), Element::CStr("")])
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
pub fn bind_msg(params: Vec<&str>) -> Vec<u8> {
    let mut msg_elements = vec![
        Element::CStr(""),
        Element::CStr(""),
        Element::Int16(1),
        Element::Int16(0), // 0 for text params
        Element::Int16(params.len() as i16),
    ];
    params.iter().for_each(|param| {
        msg_elements.push(Element::Int32(param.len() as i32));
        msg_elements.push(Element::Str(param));
    });
    msg_elements.push(Element::Int16(1));
    msg_elements.push(Element::Int16(1)); // 1 for binary results
    build_msg(Some('B'), msg_elements)
}

/**
* Int8 'E'
* Int32 Length
* CString Portal
* Int32 Max Rows, 0 = No Limit

  Portal is always anonymous, and we never limit the rows.
 */
pub fn execute_msg() -> Vec<u8> {
    build_msg(Some('E'), vec![Element::CStr(""), Element::Int32(0)])
}

/**
 * Int8 'S'
 * Int32 Length
 */
pub fn sync_msg() -> Vec<u8> {
    build_msg(Some('S'), vec![])
}

// TODO: it would be better if this could be a macro rather than a separate function
fn build_msg(msg_tag: Option<char>, elements: Vec<Element>) -> Vec<u8> {
    let content_length = elements.iter().fold(4, |acc, ele| {
        acc + match ele {
            Element::Byte(_) => 1,
            Element::Int16(_) => 2,
            Element::Int32(_) => 4,
            Element::Str(str) => str.len(),
            Element::CStr(str) => str.len() + 1,
        }
    });
    let mut msg_bytes = Vec::with_capacity(1 + content_length);
    if let Some(tag) = msg_tag {
        msg_bytes.push(tag as u8);
    }
    msg_bytes.extend_from_slice(&(content_length as i32).to_be_bytes());
    elements.iter().for_each(|ele| match ele {
        Element::Byte(byte) => msg_bytes.push(*byte),
        Element::Int16(int) => msg_bytes.extend_from_slice(&int.to_be_bytes()),
        Element::Int32(int) => msg_bytes.extend_from_slice(&int.to_be_bytes()),
        Element::Str(str) => msg_bytes.extend_from_slice(str.as_bytes()),
        Element::CStr(str) => {
            msg_bytes.extend_from_slice(str.as_bytes());
            msg_bytes.push(0);
        }
    });
    msg_bytes
}

enum Element<'a> {
    Byte(u8),
    Int16(i16),
    Int32(i32),
    Str(&'a str),
    CStr(&'a str),
}
