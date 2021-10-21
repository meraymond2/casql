use crate::cas_err::CasErr;
use crate::postgres::backend;
use crate::postgres::backend::{BackendMsg, Field};
use crate::postgres::msg_iter::MsgIter;
use crate::postgres::pg_types;
use crate::postgres::pg_types::Serialiser;
use std::collections::HashMap;
use std::io::{BufWriter, StdoutLock, Write};

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

pub struct JsonWriter<'a, T> {
    msgs: &'a mut MsgIter<'a>,
    dynamic_types: &'a HashMap<i32, String>,
    out: &'a mut T,
    is_first: bool,
}

impl<'a, T> JsonWriter<'a, T>
where
    T: Write,
{
    pub fn new(
        msgs: &'a mut MsgIter<'a>,
        dynamic_types: &'a HashMap<i32, String>,
        out: &'a mut T,
    ) -> Self {
        JsonWriter {
            msgs,
            dynamic_types,
            out,
            is_first: true,
        }
    }

    /// Iterate over the messages in the Postgres query response. First capture the field types,
    /// then parse each row according to those types and print them to the out-stream.
    pub fn write_rows(&mut self) -> Result<(), CasErr> {
        self.out.write(LEFT_BRACKET)?;

        let mut fields = Vec::new();
        while let Some(msg) = self.msgs.next() {
            match backend::type_of(&msg) {
                // BackendMsg::ErrorResponse => {} // TODO
                BackendMsg::ParseComplete => {}
                BackendMsg::ParameterDescription => {}
                BackendMsg::RowDescription => {
                    fields = backend::parse_row_desc(msg);
                }
                BackendMsg::BindComplete => {}
                BackendMsg::DataRow => {
                    if self.is_first {
                        self.is_first = false;
                    } else {
                        self.out.write(COMMA)?;
                    }
                    self.write_row(msg, &fields)?;
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
        self.out.write(RIGHT_BRACKET)?;
        self.out.write(NEW_LINE)?;
        self.out.flush().map_err(CasErr::from)
    }

    fn write_row(&mut self, msg: Vec<u8>, fields: &Vec<Field>) -> Result<(), CasErr> {
        let value_count = i16::from_be_bytes([msg[5], msg[6]]) as usize;
        let mut pos = 7; // skip discriminator (u8), msg length (i32) and value count (i32)

        self.out.write(LEFT_BRACE)?;

        for idx in 0..value_count {
            let val_len = i32::from_be_bytes([msg[pos], msg[pos + 1], msg[pos + 2], msg[pos + 3]]);
            pos += 4;
            let field = &fields[idx];
            let value = if val_len == -1 {
                None
            } else {
                let len = val_len as usize;
                let val_bytes = &msg[pos..(pos + len)];
                pos += len;
                Some(val_bytes)
            };
            self.write_key_value(field, value)?;

            if idx < value_count - 1 {
                self.out.write(COMMA)?;
            }
        }

        self.out.write(RIGHT_BRACE)?;
        Ok(())
    }

    fn write_key_value(&mut self, field: &Field, value: Option<&[u8]>) -> Result<(), CasErr> {
        self.out.write(QUOTE)?;
        self.out.write(field.name.as_bytes())?;
        self.out.write(QUOTE)?;
        self.out.write(COLON)?;
        match value {
            Some(v) => self.write_json_value(v, field.data_type_oid),
            None => {
                self.out.write("null".as_bytes())?;
                Ok(())
            }
        }
    }

    fn write_json_value(&mut self, value: &[u8], oid: i32) -> Result<(), CasErr> {
        // Using serde_json for strings to handle escaping — it’s not enough to copy the string
        // bytes straight from the Postgres message. And itoa for faster numeric writing.
        match pg_types::oid_to_serialiser(oid) {
            Serialiser::Bool => {
                let bool = if value[0] == 0 { "false" } else { "true" };
                self.out.write(bool.as_bytes())?;
            }
            Serialiser::Int16 => {
                let int = i16::from_be_bytes([value[0], value[1]]);
                itoa::write(&mut self.out, int)?;
            }
            Serialiser::Int32 => {
                let int = i32::from_be_bytes([value[0], value[1], value[2], value[3]]);
                itoa::write(&mut self.out, int)?;
            }
            Serialiser::String => {
                let str = std::str::from_utf8(value).expect("Value will be a valid UTF-8 string.");
                serde_json::to_writer(&mut self.out, str)?;
            }
            Serialiser::Unknown => {
                // If the oid isn’t a recognised constant one, check the runtime dynamic types.
                match self.dynamic_types.get(&oid) {
                    Some(typname) => {
                        // TODO, the hash map should probably be oid to enum
                        serde_json::to_writer(&mut self.out, value)?;
                    }
                    None => {
                        eprintln!("Unhandled oid {} {:?}", oid, value);
                        self.out.write("???".as_bytes())?;
                    }
                }
            }
        }
        Ok(())
    }
}

const COMMA: &[u8] = ",".as_bytes();
const QUOTE: &[u8] = "\"".as_bytes();
const COLON: &[u8] = ":".as_bytes();
const LEFT_BRACKET: &[u8] = "[".as_bytes();
const RIGHT_BRACKET: &[u8] = "]".as_bytes();
const LEFT_BRACE: &[u8] = "{".as_bytes();
const RIGHT_BRACE: &[u8] = "}".as_bytes();
const NEW_LINE: &[u8] = "\n".as_bytes();
