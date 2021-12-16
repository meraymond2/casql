use crate::binary_reader::{BinaryReader, ByteOrder};
use crate::cas_err::CasErr;
use std::io::Write;

const LEFT_SQUARE: &[u8] = "[".as_bytes();
const RIGHT_SQUARE: &[u8] = "]".as_bytes();
const COMMA: &[u8] = ",".as_bytes();
const NULL: &[u8] = "null".as_bytes();
const ZERO: &[u8] = "0".as_bytes();
const MINUS: &[u8] = "-".as_bytes();
const DECIMAL: &[u8] = ".".as_bytes();
const NAN: &[u8] = "\"NaN\"".as_bytes();
const INFINITY: &[u8] = "\"Infinity\"".as_bytes();
const NEGATIVE_INFINITY: &[u8] = "\"-Infinity\"".as_bytes();

/// Given:
/// u8: 0/1, false/true
/// Writes:
/// true/false
pub fn serialise_bool<Out>(bytes: &[u8], out: &mut Out) -> Result<(), CasErr>
where
    Out: Write,
{
    if bytes[0] == 1 {
        out.write("true".as_bytes())?;
    } else {
        out.write("false".as_bytes())?;
    }
    Ok(())
}

/// Given:
/// i16: value
/// Writes:
/// a number
pub fn serialise_i16<Out>(bytes: &[u8], out: &mut Out) -> Result<(), CasErr>
where
    Out: Write,
{
    let int = i16::from_be_bytes([bytes[0], bytes[1]]);
    itoap::write(out, int)?;
    Ok(())
}

/// Given:
/// i32: value
/// Writes:
/// a number
pub fn serialise_i32<Out>(bytes: &[u8], out: &mut Out) -> Result<(), CasErr>
where
    Out: Write,
{
    let int = i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    itoap::write(out, int)?;
    Ok(())
}

/// Given:
/// i64: value
/// Writes:
/// a number
pub fn serialise_i64<Out>(bytes: &[u8], out: &mut Out) -> Result<(), CasErr>
where
    Out: Write,
{
    let int = i64::from_be_bytes([
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
    ]);
    itoap::write(out, int)?;
    Ok(())
}

/// Given:
/// f32: value
/// Writes:
/// a number, or "NaN" or "Infinity" or "-Infinity"
///
/// JSON doesn't include a way to indicate NaN, Infinity or -Infinity because JSON numbers
/// aren't tied to any particular implementation of number, floating point or otherwise.
///
/// serde_json serialises them to null, which makes sense, but for casql I would prefer to
/// retain more information, so I have decided to write them as strings for now. Different
/// languages will be able to parse different strings as floats, so this may cause issues.
pub fn serialise_f32<Out>(bytes: &[u8], out: &mut Out) -> Result<(), CasErr>
where
    Out: Write,
{
    let float = f32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    if float.is_finite() {
        let mut ryu_buf = ryu::Buffer::new();
        let str = ryu_buf.format(float);
        out.write(str.as_bytes())?;
    } else if float.is_nan() {
        out.write(NAN)?;
    } else if float.is_infinite() {
        if float.is_sign_negative() {
            out.write(NEGATIVE_INFINITY)?;
        } else {
            out.write(INFINITY)?;
        }
    }
    Ok(())
}

/// Given:
/// f64: value
/// Writes:
/// a number, or "NaN" or "Infinity" or "-Infinity"
///
/// JSON doesn't include a way to indicate NaN, Infinity or -Infinity because JSON numbers
/// aren't tied to any particular implementation of number, floating point or otherwise.
///
/// serde_json serialises them to null, which makes sense, but for casql I would prefer to
/// retain more information, so I have decided to write them as strings for now. Different
/// languages will be able to parse different strings as floats, so this may cause issues.
pub fn serialise_f64<Out>(bytes: &[u8], out: &mut Out) -> Result<(), CasErr>
where
    Out: Write,
{
    let float = f64::from_be_bytes([
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
    ]);
    if float.is_finite() {
        let mut ryu_buf = ryu::Buffer::new();
        let str = ryu_buf.format(float);
        out.write(str.as_bytes())?;
    } else if float.is_nan() {
        out.write(NAN)?;
    } else if float.is_infinite() {
        if float.is_sign_negative() {
            out.write(NEGATIVE_INFINITY)?;
        } else {
            out.write(INFINITY)?;
        }
    }
    Ok(())
}

/// Given:
/// i32: block
/// i16: offset
///
/// Writes a two-element array representing the (block, offset) tuple.
pub fn serialise_tid<Out>(bytes: &[u8], out: &mut Out) -> Result<(), CasErr>
where
    Out: Write,
{
    let block = i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    let offset = i16::from_be_bytes([bytes[4], bytes[5]]);
    out.write(LEFT_SQUARE)?;
    itoap::write(&mut *out, block)?;
    out.write(COMMA)?;
    itoap::write(&mut *out, offset)?;
    out.write(RIGHT_SQUARE)?;
    Ok(())
}

/// Given:
/// i16: # of digit-blocks specified, i.e. non-zero digits
/// i16: weight, # of digit-blocks left of the decimal point, 0-indexed
/// i16: flag, magic numbers indicating negative or NaN or NULL
/// i16: scale, number of specified decimals places
/// i16[]: digit-blocks, a number representing 4 digits
/// Writes:
/// a number
pub fn serialise_bignum<Out>(bytes: &[u8], out: &mut Out) -> Result<(), CasErr>
where
    Out: Write,
{
    let mut bignum = BinaryReader::from(bytes, ByteOrder::BigEndian);
    let mut digits = bignum.i16();
    let mut weight = bignum.i16();
    let flag = bignum.i16();
    let mut scale = bignum.i16();

    // https://github.com/postgres/postgres/blob/5d08137076fd39694188ec4625013756aab889e1/src/interfaces/ecpg/include/pgtypes_numeric.h#L6
    match flag {
        0x4000 => {
            // Add a negative sign
            out.write(MINUS)?;
        }
        -16384 => {
            // 0xC000
            out.write(NAN)?;
            return Ok(());
        }
        -4096 => {
            // 0xF000
            out.write(NULL)?;
            return Ok(());
        }
        _ => {}
    }
    // If it’s zero, just write zero.
    if digits == 0 {
        out.write(ZERO)?;
        return Ok(());
    }

    // Write integral part.
    if weight >= 0 {
        // Write the first block without leading zeros.
        let first_block = bignum.i16();
        itoap::write(&mut (*out), first_block)?;
        digits -= 1;
        weight -= 1;
        // Write subsequent integral blocks as zero-padded.
        while weight >= 0 {
            let block = if digits > 0 { bignum.i16() } else { 0 };
            write!(out, "{:04}", block)?;
            digits -= 1;
            weight -= 1;
        }
    } else {
        out.write(ZERO)?;
    }
    // Write fractional part.
    if scale > 0 {
        out.write(DECIMAL)?;
    } else {
        return Ok(());
    }
    // Add leading zeros if necessary, i.e. if the first digit block is more than 4 zeros
    // after the decimal. If we’ve just written an integral part, the weight will be -1.
    let zero_block_count = -1 - weight;
    for _ in 0..zero_block_count {
        out.write("0000".as_bytes())?;
        scale -= 4;
    }
    // Write blocks with leading and trailing zeros if present.
    while scale > 4 {
        let block = if digits > 0 { bignum.i16() } else { 0 };
        write!(out, "{:04}", block)?;
        digits -= 1;
        scale -= 4;
    }
    // Write final block, with leading but without trailing zeros.
    if scale > 0 {
        let block = if digits > 0 { bignum.i16() } else { 0 };
        let digits = format!("{:04}", block);
        let trimmed = &digits.as_bytes()[0..(scale as usize)];
        out.write(trimmed)?;
    }
    Ok(())
}
