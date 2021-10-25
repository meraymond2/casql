use crate::cas_err::CasErr;
use crate::postgres::backend_msgs;
use crate::postgres::postgis::ewkb::EWKB;
use crate::postgres::row_iter::RowIter;
use serde::ser::SerializeSeq;
use serde::ser::Serializer;
use std::io::{BufWriter, Write};

pub fn write_json(rows: RowIter) -> Result<(), CasErr> {
    let stdout = std::io::stdout();
    let handle = stdout.lock();
    let mut buf_writer = BufWriter::new(handle);
    let mut ser = serde_json::Serializer::new(&mut buf_writer);
    let mut seq = ser.serialize_seq(None)?;
    for row in rows {
        seq.serialize_element(&row)?;
    }
    seq.end()?;
    buf_writer.write("\n".as_bytes())?;
    buf_writer.flush()?;
    Ok(())
}
