use crate::cas_err::CasErr;
use crate::postgres::backend_msgs;
use crate::postgres::backend_msgs::{BackendMsg, Field};
use crate::postgres::msg_iter::MsgIter;

// The messages arrive from Postgres in the following order:
// ParseComplete
// ParameterDescription
// RowDescription
// BindComplete
// DataRow...
// Close
// ReadyForQuery
pub struct RowIter<'msgs> {
    msgs: &'msgs mut MsgIter<'msgs>,
    pub fields: Vec<Field>,
}

impl<'msgs> RowIter<'msgs> {
    pub fn from(msgs: &'msgs mut MsgIter<'msgs>) -> Result<Self, CasErr> {
        let mut fields = Vec::new();
        while let Some(msg) = msgs.next() {
            match backend_msgs::type_of(&msg) {
                BackendMsg::ErrorResponse => {
                    let err_msg = backend_msgs::parse_error_response(&msg);
                    Err(CasErr::PostgresErr(err_msg.to_string()))?;
                }
                BackendMsg::ParseComplete => {}
                BackendMsg::ParameterDescription => {}
                BackendMsg::RowDescription => {
                    fields = backend_msgs::parse_row_desc(&msg);
                }
                BackendMsg::BindComplete => {
                    // This should come just before the first data row.
                    break;
                }
                _ => {
                    eprintln!("Received unexpected message from Postgres: {:?}", msg);
                }
            }
        }
        Ok(RowIter { msgs, fields })
    }
}

impl<'msgs> Iterator for RowIter<'msgs> {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        self.msgs
            .next()
            .and_then(|msg| match backend_msgs::type_of(&msg) {
                BackendMsg::DataRow => Some(msg),
                BackendMsg::Close => self.next(),
                BackendMsg::ReadyForQuery => None, // finished
                _ => {
                    eprintln!("Received unexpected message from Postgres: {:?}", msg);
                    None
                }
            })
    }
}
