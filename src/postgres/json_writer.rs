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

It would perhaps be more elegant to model this as a series of transformations on the message
iterator, parsing the binary message to a struct, parsing the values within that struct, serialising
the values to JSON and finally writing it the output.

However, in order to minimise copying of data, I’m preferring a unidirectional flow, which will
allow me to copy directly from the response buffer to stdout, without complicated lifetimes. It
would make it harder to test, had any intention of writing unit tests.

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

pub fn write_json_rows(msgs: &mut MsgIter) {
    let stdout = std::io::stdout();
    let handle = stdout.lock();
    let out = std::io::BufWriter::new(handle);
    let mut writer = JsonWriter { out };
    let mut first = true;

    writer.left_square_bracket();

    while let Some(msg) = msgs.next() {
        match backend::type_of(&msg) {
            BackendMsg::ErrorResponse => {} // TODO
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
                                writer.comma();
                            }
                            write_row(msg, &fields, &mut writer);
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
    writer.right_square_bracket();
    writer.end();
}

/// Write row as JSON object, where row is the bytes representing a DataRow Postgres message.
fn write_row<T>(row: Vec<u8>, fields: &Vec<Field>, writer: &mut JsonWriter<T>)
where
    T: Write,
{
    let value_count = i16::from_be_bytes([row[5], row[6]]) as usize;
    let mut pos = 7; // skip discriminator (u8), msg length (i32) and value count (i32)

    writer.left_curly();

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
        writer.key_value(field, value);

        if idx < value_count - 1 {
            writer.comma();
        }
    }

    writer.right_curly();
}

struct JsonWriter<Out> {
    out: Out,
}

impl<Out> JsonWriter<Out>
where
    Out: Write,
{
    fn key_value(&mut self, field: &Field, value: Option<&[u8]>) {
        self.string(field.name.as_bytes());
        self.out.write(":".as_bytes()).unwrap();
        match value {
            Some(v) => {
                self.value(v, field.data_type_oid);
            }
            None => {
                self.out.write("null".as_bytes()).unwrap();
            }
        }
    }

    fn string(&mut self, s: &[u8]) {
        self.out.write("\"".as_bytes()).unwrap();
        self.out.write(s).unwrap();
        self.out.write("\"".as_bytes()).unwrap();
    }

    fn left_square_bracket(&mut self) {
        self.out.write("[".as_bytes()).unwrap();
    }

    fn right_square_bracket(&mut self) {
        self.out.write("]".as_bytes()).unwrap();
    }

    fn left_curly(&mut self) {
        self.out.write("{".as_bytes()).unwrap();
    }

    fn right_curly(&mut self) {
        self.out.write("}".as_bytes()).unwrap();
    }

    fn comma(&mut self) {
        self.out.write(",".as_bytes()).unwrap();
    }

    fn end(&mut self) {
        self.out.write("\n".as_bytes()).unwrap();
        self.out.flush().unwrap();
    }

    fn value(&mut self, value: &[u8], oid: i32) {
        match pg_types::oid_to_serialiser(oid) {
            Serialiser::Bool => {
                let s = if value[0] == 0 { "false" } else { "true" };
                self.out.write(s.as_bytes()).unwrap();
            }
            Serialiser::Int16 => {
                let int = i16::from_be_bytes([value[0], value[1]]);
                self.out.write(int.to_string().as_bytes()).unwrap();
            }
            Serialiser::Int32 => {
                let int = i32::from_be_bytes([value[0], value[1], value[2], value[3]]);
                self.out.write(int.to_string().as_bytes()).unwrap();
            }
            Serialiser::String => self.string(value),
            Serialiser::Unknown => {
                eprintln!("Unhandled oid {} {:?}", oid, value);
                self.string("???".as_bytes());
            }
        };
    }
}
