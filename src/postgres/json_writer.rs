use crate::cas_err::CasErr;
use crate::postgres::backend;
use crate::postgres::backend::{BackendMsg, Field};
use crate::postgres::msg_iter::MsgIter;
use crate::postgres::pg_types;
use crate::postgres::pg_types::Serialiser;
use std::io::Write;

/*
Given an iterator containing the raw Postgres responses from the query, we need to parse the
RowDescription — which will tell us the field names and types for each value — and then parse the
DataRows, serialise them as JSON, and write them to the output.

This could also be modelled as a series of transformations on the message iterator, but the result
isn’t any easier to read, so I’ve kept it quite imperative.

The messages arrive from Postgres in the following order:
ParseComplete
ParameterDescription
RowDescription
BindComplete
DataRow
DataRow...
Close
ReadyForQuery
*/

pub fn write_json_rows(msgs: &mut MsgIter) -> Result<(), CasErr> {
    let stdout = std::io::stdout();
    let handle = stdout.lock();
    let mut out = std::io::BufWriter::new(handle);
    let mut first = true;

    out.write(LEFT_BRACKET)?;

    while let Some(msg) = msgs.next() {
        match backend::type_of(&msg) {
            // BackendMsg::ErrorResponse => {} // TODO
            BackendMsg::ParseComplete => {}
            BackendMsg::ParameterDescription => {}
            BackendMsg::RowDescription => {
                let fields = backend::parse_row_desc(msg);
                while let Some(msg) = msgs.next() {
                    match backend::type_of(&msg) {
                        BackendMsg::BindComplete => {}
                        BackendMsg::DataRow => {
                            if first {
                                first = false;
                            } else {
                                out.write(COMMA)?;
                            }
                            write_row(msg, &fields, &mut out)?;
                        }
                        BackendMsg::Close => {}
                        BackendMsg::ReadyForQuery => {
                            break;
                        }
                        _ => {
                            eprintln!("Received unexpected message from Postgres: {:?}", msg);
                        }
                    }
                }
                break;
            }
            _ => {
                eprintln!("Received unexpected message from Postgres: {:?}", msg);
            }
        }
    }
    out.write(RIGHT_BRACKET)?;
    out.write(NEW_LINE)?;
    out.flush().map_err(CasErr::from)
}

/// Write DataRow message as a JSON object.
fn write_row<T>(row: Vec<u8>, fields: &Vec<Field>, out: &mut T) -> Result<(), CasErr>
where
    T: Write,
{
    let value_count = i16::from_be_bytes([row[5], row[6]]) as usize;
    let mut pos = 7; // skip discriminator (u8), msg length (i32) and value count (i32)

    out.write(LEFT_BRACE)?;

    for idx in 0..value_count {
        let val_len = i32::from_be_bytes([row[pos], row[pos + 1], row[pos + 2], row[pos + 3]]);
        pos += 4;

        let field = &fields[idx];
        let value = if val_len == -1 {
            None
        } else {
            let len = val_len as usize;
            let val_bytes = &row[pos..(pos + len)];
            pos += len;
            Some(val_bytes)
        };
        write_key_value(field, value, out)?;

        if idx < value_count - 1 {
            out.write(COMMA)?;
        }
    }

    out.write(RIGHT_BRACE)?;
    Ok(())
}

fn write_key_value<T>(field: &Field, value: Option<&[u8]>, out: &mut T) -> Result<(), CasErr>
where
    T: Write,
{
    out.write(QUOTE)?;
    out.write(field.name.as_bytes())?;
    out.write(QUOTE)?;
    out.write(COLON)?;
    match value {
        Some(v) => write_value(v, field.data_type_oid, out),
        None => {
            out.write("null".as_bytes())?;
            Ok(())
        }
    }
}

fn write_value<T>(value: &[u8], oid: i32, out: &mut T) -> Result<(), CasErr>
where
    T: Write,
{
    match pg_types::oid_to_serialiser(oid) {
        Serialiser::Bool => {
            let bool = if value[0] == 0 { "false" } else { "true" };
            out.write(bool.as_bytes())?;
        }
        Serialiser::Int16 => {
            let int = i16::from_be_bytes([value[0], value[1]]);
            itoa::write(out, int)?;
        }
        Serialiser::Int32 => {
            let int = i32::from_be_bytes([value[0], value[1], value[2], value[3]]);
            itoa::write(out, int)?;
        }
        Serialiser::String => {
            let str = std::str::from_utf8(value).expect("Value will be a valid UTF-8 string.");
            serde_json::to_writer(out, str)?;
        }
        Serialiser::Unknown => {
            eprintln!("Unhandled oid {} {:?}", oid, value);
            out.write("???".as_bytes())?;
        }
    }
    Ok(())
}

const COMMA: &[u8] = ",".as_bytes();
const QUOTE: &[u8] = "\"".as_bytes();
const COLON: &[u8] = ":".as_bytes();
const LEFT_BRACKET: &[u8] = "[".as_bytes();
const RIGHT_BRACKET: &[u8] = "]".as_bytes();
const LEFT_BRACE: &[u8] = "{".as_bytes();
const RIGHT_BRACE: &[u8] = "}".as_bytes();
const NEW_LINE: &[u8] = "\n".as_bytes();
