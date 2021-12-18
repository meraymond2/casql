use crate::cas_err::CasErr;
use std::io::Write;

const NULL: &[u8] = "null".as_bytes();
const TRUE: &[u8] = "true".as_bytes();
const FALSE: &[u8] = "false".as_bytes();

// TODO: numbers

// This is arguably a waste of cycles, but it bothers me that the whitespace for json is off, so
// here we are.

/// Given:
/// u8[]: bytes representing UTF-8 characters of a JSON string
/// Writes:
/// a JSON string, with the white space removed
pub fn serialise_json<Out>(bytes: &[u8], out: &mut Out) -> Result<(), CasErr>
where
    Out: Write,
{
    let mut pos = 0;
    while pos < bytes.len() {
        let byte = bytes[pos];
        if byte == 1 {
            // jsonb strings start with SOH
            pos += 1;
        } else if byte == 34 {
            let start = pos;
            pos += 1; // opening quote
            let mut escapes = 0;
            while !(bytes[pos] == 34 && escapes % 2 == 0) {
                if bytes[pos] == 92 {
                    escapes += 1;
                } else {
                    escapes = 0;
                }
                pos += 1;
            }
            pos += 1; // closing quote
            out.write(&bytes[start..pos])?;
        } else if byte == 44 || byte == 58 || byte == 91 || byte == 93 || byte == 123 || byte == 125
        {
            // colon, comma, left-square, right-square, left-brace, right-brace
            out.write(&[byte])?;
            pos += 1;
        } else if byte == 102 {
            // f
            out.write(FALSE)?;
            pos += 5;
        } else if byte == 110 {
            // n
            out.write(NULL)?;
            pos += 4;
        } else if byte == 116 {
            // t
            out.write(TRUE)?;
            pos += 4;
        } else {
            pos += 1;
        }
    }
    Ok(())
}

fn numericish(byte: u8) -> bool {
    (byte >= 48 && byte <= 57)
        || byte == 46 // .
        || byte == 45 // -
        || byte == 43 // +
        || byte == 69 // E
        || byte == 101 // e
}
