use crate::cas_err::CasErr;
use std::io::Write;

const DOUBLE_QUOTE: &[u8] = "\"".as_bytes();

/// Given:
/// u8[]: bytes representing UTF-8 characters
/// Writes:
/// a JSON string
pub fn serialise_str<Out>(bytes: &[u8], out: &mut Out) -> Result<(), CasErr>
where
    Out: Write,
{
    // I’m curious about the performance implications for building up a second string versus writing
    // everything straight to the buffered writer. I assume the former is slower, but I’m not sure
    // if the Result checking adds overhead. For the time being, I’m using a second string because
    // I’m too lazy to figure out how to write a char directly, and at least there aren’t many
    // allocations.
    // Need to confirm to be completely sure, but it sounds like I can just check the ascii bytes
    // I’m interested in, because they won’t overlap with UTF-8 code points, and then skips chars.
    let s = std::str::from_utf8(bytes).expect("Strings must be valid UTF-8.");
    let mut escaped = String::with_capacity(bytes.len());
    for char in s.chars() {
        match char {
            '\\' => {
                escaped.push('\\');
                escaped.push('\\');
            }
            '"' => {
                escaped.push('\\');
                escaped.push('"');
            }
            '\n' => {
                escaped.push('\\');
                escaped.push('n');
            }
            '\r' => {
                escaped.push('\\');
                escaped.push('r');
            }
            '\t' => {
                escaped.push('\\');
                escaped.push('t');
            }
            _ => escaped.push(char),
        }
    }
    out.write(DOUBLE_QUOTE)?;
    out.write(&escaped.as_bytes())?;
    out.write(DOUBLE_QUOTE)?;
    Ok(())
}
