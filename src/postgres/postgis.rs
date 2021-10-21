use crate::postgres::backend;
use crate::postgres::backend::{BackendMsg, BinaryMsg};
use crate::postgres::msg_iter::MsgIter;

// Not all of them, just the external types
pub const POSTGIS_TYPES: [&'static str; 5] =
    ["geometry", "geography", "box2d", "box3d", "geometry_dump"];

pub const POSTGIS_QUERY: &'static str =  "SELECT typname, oid FROM pg_type WHERE typname IN ('geometry', 'geography', 'box2d', 'box3d', 'geometry_dump')";

#[derive(Debug)]
pub struct PgType {
    pub name: String,
    pub oid: i32,
}

pub fn parse_type_lookup(resp: &mut MsgIter) -> Vec<PgType> {
    let mut pg_types = Vec::with_capacity(POSTGIS_TYPES.len());
    while let Some(msg) = resp.next() {
        match backend::type_of(&msg) {
            // BackendMsg::ErrorResponse => {} // TODO
            BackendMsg::ParseComplete => {}
            BackendMsg::ParameterDescription => {}
            BackendMsg::RowDescription => {}
            BackendMsg::BindComplete => {}
            BackendMsg::DataRow => {
                pg_types.push(parse_type_lookup_row(msg));
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
    pg_types
}

fn parse_type_lookup_row(msg: Vec<u8>) -> PgType {
    let mut msg = BinaryMsg::from(msg);
    // skip discriminator and message size
    msg.skip(5);
    // skip value count — it’s always 2, name and oid
    msg.skip(2); // (i16)

    let name_len = msg.i32();
    let name_bytes = msg.bytes(name_len as usize);
    let name = std::str::from_utf8(&name_bytes)
        .expect("Value will be a valid UTF-8 string.")
        .to_owned();
    msg.skip(4); // skip oid length, it’s always 4
    let oid = msg.i32();
    PgType { name, oid }
}
