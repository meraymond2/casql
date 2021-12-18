use crate::cas_err::CasErr;
use std::io::Write;

const ESCAPE: &[u8] = "\\".as_bytes();
const FORWARD_SLASH: &[u8] = "\\".as_bytes();
const DOUBLE_QUOTE: &[u8] = "\"".as_bytes();

/// Given:
/// u8[]: bytes representing UTF-8 characters
/// Writes:
/// a JSON string
pub fn serialise_str<Out>(bytes: &[u8], out: &mut Out) -> Result<(), CasErr>
where
    Out: Write,
{
    out.write(DOUBLE_QUOTE)?;
    let mut pos = 0;
    while pos < bytes.len() {
        if is_escaped(bytes[pos]) {
            match bytes[pos] {
                1 => {}
                9 => {
                    out.write(ESCAPE)?;
                    out.write("t".as_bytes())?;
                }
                10 => {
                    out.write(ESCAPE)?;
                    out.write("n".as_bytes())?;
                }
                13 => {
                    out.write(ESCAPE)?;
                    out.write("r".as_bytes())?;
                }
                34 => {
                    out.write(ESCAPE)?;
                    out.write(DOUBLE_QUOTE)?;
                }
                92 => {
                    out.write(ESCAPE)?;
                    out.write(FORWARD_SLASH)?;
                }
                _ => unreachable!(),
            }
            pos += 1;
        } else {
            let start = pos;
            while pos < bytes.len() && !is_escaped(bytes[pos]) {
                pos += 1;
            }
            out.write(&bytes[start..pos])?;
        }
    }
    out.write(DOUBLE_QUOTE)?;
    Ok(())
}

/// Returns true if the byte is a character that should be escaped in the JSON string, i.e. it is a
/// \t, \n, \r, " or \. Some string-like sequences in Postgres also start with a SOH char, which is
/// stripped.
fn is_escaped(byte: u8) -> bool {
    byte == 1 || byte == 9 || byte == 10 || byte == 13 || byte == 34 || byte == 92
}
