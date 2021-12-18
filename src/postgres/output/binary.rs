use crate::binary_reader::{BinaryReader, ByteOrder};
use crate::cas_err::CasErr;
use std::io::Write;

const DOUBLE_QUOTE: &[u8] = "\"".as_bytes();
const LEFT_SQUARE: &[u8] = "[".as_bytes();
const RIGHT_SQUARE: &[u8] = "]".as_bytes();
const COMMA: &[u8] = ",".as_bytes();

/// Given:
/// i32: # of bits in string
/// u8[]: bytes representing octets, the last of which may only represent a partial octet,
///       e.g. 64 (10000000) may represent "1" if there’s only one bit remaining
/// Writes:
/// a string composed of 1s and 0s
pub fn serialise_bitstring<Out>(bytes: &[u8], out: &mut Out) -> Result<(), CasErr>
where
    Out: Write,
{
    let mut bit_str = BinaryReader::from(bytes, ByteOrder::BigEndian);
    let mut bit_len = bit_str.i32();
    out.write(DOUBLE_QUOTE)?;
    while bit_len >= 8 {
        let block = bit_str.u8();
        write!(out, "{:08b}", block)?;
        bit_len -= 8;
    }
    if bit_len > 0 {
        let block = bit_str.u8();
        let bits = format!("{:08b}", block);
        let trimmed = &bits.as_bytes()[0..(bit_len as usize)];
        out.write(trimmed)?;
    }
    out.write(DOUBLE_QUOTE)?;
    Ok(())
}

/// Given:
/// u8[]: bytes
/// Writes:
/// an array of decimal numbers
pub fn serialise_bytes<Out>(bytes: &[u8], out: &mut Out) -> Result<(), CasErr>
where
    Out: Write,
{
    out.write(LEFT_SQUARE)?;
    let mut first = true;
    for byte in bytes {
        if first {
            first = false;
        } else {
            out.write(COMMA)?;
        }
        itoap::write(&mut (*out), *byte)?;
    }
    out.write(RIGHT_SQUARE)?;
    Ok(())
}

/// From https://datatracker.ietf.org/doc/html/rfc4122#section-4.1.2
/// Given:
/// u32: the low field of the timestamp
/// u16: the middle field of the timestamp
/// u16: the high field of the timestamp multiplexed with the version number
/// u8: the high field of the clock sequence multiplexed with the variant
/// u8: the low field of the clock sequence
/// u48: the spatially unique node identifier
///
/// Writes:
/// a lowercase UUID string
pub fn serialise_uuid<Out>(bytes: &[u8], out: &mut Out) -> Result<(), CasErr>
where
    Out: Write,
{
    let mut uuid = BinaryReader::from(bytes, ByteOrder::BigEndian);
    write!(
        out,
        "\"{:x}-{:x}-{:x}-{:x}{:x}-{:x}{:x}\"",
        uuid.i32(),
        uuid.i16(),
        uuid.i16(),
        uuid.u8(),
        uuid.u8(),
        uuid.i16(),
        uuid.i32()
    )?;
    Ok(())
}
