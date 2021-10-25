
#[derive(Debug)]
pub enum CasVal {
    Bool(bool),
    Null,
    Int16(i16),
    Int32(i32),
    Str(String),
    Unparsed,
}