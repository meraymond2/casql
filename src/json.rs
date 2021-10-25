use crate::cas_err::CasErr;
use crate::postgres::backend_msgs;
use crate::postgres::row_iter::RowIter;

// TODO: Can I parse the data row within the RowIter, to avoid leaking the Postgres parsing outside of that module?
// there is an issue that I can't return something that references the iterator Item, something like all items have to
// have the same lifetime, whereas I'm trying to only use one item at a time. Maybe a struct that does the same thing
// but isn't officially an iterator. I want a stream, not an iter. I'd also like to solve the partial borrow thing.
// This is ok for now though
pub fn write_json(rows: RowIter) -> Result<(), CasErr> {
    // let fields = rows.fields.clone();
    // let types = rows.dynamic_types.clone();
    for row in rows {
        // let parsed = backend_msgs::parse_data_row(row, &fields, &types);
        println!("ROW: {:?}", row);
    }
    Ok(())
}
