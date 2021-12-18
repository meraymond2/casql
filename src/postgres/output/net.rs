use crate::binary_reader::{BinaryReader, ByteOrder};
use crate::cas_err::CasErr;
use std::io::Write;

/// Given:
/// u8[]: 6 or 8 bytes representing the MAC address, for macaddr and macaddr8 respectively
///
/// Writes:
/// a string of 6 or 8 hex numbers separated by hyphens per the IEEE 802 standard.
pub fn serialise_mac_addr<Out>(bytes: &[u8], out: &mut Out) -> Result<(), CasErr>
where
    Out: Write,
{
    if bytes.len() == 6 {
        write!(
            out,
            "\"{:02x}-{:02x}-{:02x}-{:02x}-{:02x}-{:02x}\"",
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5]
        )?;
    } else {
        write!(
            out,
            "\"{:02x}-{:02x}-{:02x}-{:02x}-{:02x}-{:02x}-{:02x}-{:02x}\"",
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]
        )?;
    }
    Ok(())
}

/// Given:
/// u8: family, PGSQL_AF_INET or PGSQL_AF_INET6, 2 or 3
/// u8: number of bits in netmask
/// u8: inet/cidr, 0 for inet, 1 for cidr (as far as I can tell)
/// u8: length, 4 or 16 for v4 and v6 respectively
/// u8[] or u16[]: the address
///
/// Writes:
/// A network address string. IPv4 addresses are written as four decimal numbers separated by
/// periods. IPv6 addresses are written as hex digits separated by colons. Zero compression is _not_
/// applied, but leading zeros are omitted, see https://datatracker.ietf.org/doc/html/rfc5952#section-2.1.
///
/// For IP addresses, the netmask_bits are omitted if it represents a single address, that is if
/// they are 32 for v4 or 128 for v6.
pub fn serialise_inet<Out>(bytes: &[u8], out: &mut Out) -> Result<(), CasErr>
where
    Out: Write,
{
    let mut inet = BinaryReader::from(bytes, ByteOrder::BigEndian);
    let _family = inet.u8();
    let netmask_bits = inet.u8();
    let is_addr = inet.u8() == 0;
    let length = inet.u8();
    if length == 4 {
        if netmask_bits == 32 && is_addr {
            write!(
                out,
                "\"{}.{}.{}.{}\"",
                bytes[4], bytes[5], bytes[6], bytes[7]
            )?;
        } else {
            write!(
                out,
                "\"{}.{}.{}.{}/{}\"",
                bytes[4], bytes[5], bytes[6], bytes[7], netmask_bits
            )?;
        }
    } else {
        if netmask_bits == 128 && is_addr {
            write!(
                out,
                "\"{:x}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}\"",
                inet.i16(),
                inet.i16(),
                inet.i16(),
                inet.i16(),
                inet.i16(),
                inet.i16(),
                inet.i16(),
                inet.i16(),
            )?;
        } else {
            write!(
                out,
                "\"{:x}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}/{}\"",
                inet.i16(),
                inet.i16(),
                inet.i16(),
                inet.i16(),
                inet.i16(),
                inet.i16(),
                inet.i16(),
                inet.i16(),
                netmask_bits
            )?;
        }
    }
    Ok(())
}
