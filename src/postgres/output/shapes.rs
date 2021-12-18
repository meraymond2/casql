use crate::binary_reader::{BinaryReader, ByteOrder};
use crate::cas_err::CasErr;
use std::io::Write;

const LEFT_SQUARE: &[u8] = "[".as_bytes();
const RIGHT_SQUARE: &[u8] = "]".as_bytes();
const COMMA: &[u8] = ",".as_bytes();

/// Given:
/// f64: x
/// f64: y
///
/// Writes:
/// coordinate array [x, y]
pub fn serialise_point<Out>(bytes: &[u8], out: &mut Out) -> Result<(), CasErr>
where
    Out: Write,
{
    let mut point = BinaryReader::from(bytes, ByteOrder::BigEndian);
    write!(out, "[{},{}]", point.f64(), point.f64())?;
    Ok(())
}

/// Given:
/// Representing a linear equation that describes a line:
/// f64: A - x coefficient
/// f64: B - y coefficient
/// f64: C - constant
///
/// Writes:
/// A string representation of the linear equation, "Ax + By + C = 0".
pub fn serialise_line<Out>(bytes: &[u8], out: &mut Out) -> Result<(), CasErr>
where
    Out: Write,
{
    let mut line = BinaryReader::from(bytes, ByteOrder::BigEndian);
    write!(
        out,
        "\"{}x + {}y + {} = 0\"",
        line.f64(),
        line.f64(),
        line.f64()
    )?;
    Ok(())
}

/// Given:
/// f64: x1
/// f64: y1
/// f64: x2
/// f64: y2
///
/// Writes:
/// [[x1, y1],[x2, y2]]
pub fn serialise_line_segment<Out>(bytes: &[u8], out: &mut Out) -> Result<(), CasErr>
where
    Out: Write,
{
    let mut lseg = BinaryReader::from(bytes, ByteOrder::BigEndian);
    write!(
        out,
        "[[{},{}],[{},{}]]",
        lseg.f64(),
        lseg.f64(),
        lseg.f64(),
        lseg.f64()
    )?;
    Ok(())
}

/// Given:
/// f64: centre x
/// f64: centre y
/// f64: radius
///
/// Writes:
/// [[x, y], radius]
pub fn serialise_circle<Out>(bytes: &[u8], out: &mut Out) -> Result<(), CasErr>
where
    Out: Write,
{
    let mut circle = BinaryReader::from(bytes, ByteOrder::BigEndian);
    write!(
        out,
        "[[{},{}],{}]",
        circle.f64(),
        circle.f64(),
        circle.f64(),
    )?;
    Ok(())
}

/// Given:
/// f64: x1
/// f64: y1
/// f64: x2
/// f64: y2
///
/// Writes:
/// [[x1, y1],[x2, y2]] where x1,y1 is the upper right point, and x2,y2 is the lower left point.
pub fn serialise_box<Out>(bytes: &[u8], out: &mut Out) -> Result<(), CasErr>
where
    Out: Write,
{
    let mut box_pts = BinaryReader::from(bytes, ByteOrder::BigEndian);
    write!(
        out,
        "[[{},{}],[{},{}]]",
        box_pts.f64(),
        box_pts.f64(),
        box_pts.f64(),
        box_pts.f64()
    )?;
    Ok(())
}

/// Given:
/// i32: point count
/// [f64, f64]: points
///
/// Writes:
/// [[f64, f64], ...].
pub fn serialise_polygon<Out>(bytes: &[u8], out: &mut Out) -> Result<(), CasErr>
where
    Out: Write,
{
    let mut poly = BinaryReader::from(bytes, ByteOrder::BigEndian);
    let count = poly.i32();
    write_coords(&mut poly, &[count, 2], out)?;
    Ok(())
}

/// Given:
/// u8: 0 closed or 1 open path
/// i32: point count
/// [f64, f64]: points
///
/// Writes:
/// [[f64, f64], ...] Unlike psql, doesn’t distinguish between open and closed paths, and both are
/// printed as an array of points.
pub fn serialise_path<Out>(bytes: &[u8], out: &mut Out) -> Result<(), CasErr>
where
    Out: Write,
{
    let mut path = BinaryReader::from(bytes, ByteOrder::BigEndian);
    let _open = path.u8() == 1;
    let count = path.i32();
    write_coords(&mut path, &[count, 2], out)?;
    Ok(())
}

/// Like write_array_elements, but the size isn’t encoded, because they’re always f64s.
fn write_coords<Out>(
    bytes: &mut BinaryReader,
    dimensions: &[i32],
    out: &mut Out,
) -> Result<(), CasErr>
where
    Out: Write,
{
    if dimensions.len() == 1 {
        let mut first = true;
        out.write(LEFT_SQUARE)?;
        for _ in 0..dimensions[0] {
            if first {
                first = false
            } else {
                out.write(COMMA)?;
            }
            write!(out, "{}", bytes.f64())?;
        }
        out.write(RIGHT_SQUARE)?;
    } else {
        let mut first = true;
        out.write(LEFT_SQUARE)?;
        for _ in 0..dimensions[0] {
            if first {
                first = false
            } else {
                out.write(COMMA)?;
            }
            write_coords(bytes, &dimensions[1..dimensions.len()], out)?;
        }
        out.write(RIGHT_SQUARE)?;
    }
    Ok(())
}
