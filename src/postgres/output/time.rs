use crate::cas_err::CasErr;
use std::io::Write;
use std::ops::Add;

const DOUBLE_QUOTE: &[u8] = "\"".as_bytes();
//             // Dates are stored as an i32, representing days after 2000-01-01. The Rust time libraries
//             // only handle +/- 10000 years. Calculating future dates is easy enough, but I’m not as sure
//             // about historical dates. Since Postgres only goes back to 4713 BC, I can use the time
//             // crate for historical dates.
/// Given:
/// i32: days after 2000-01-01, negative numbers are days before
///
/// Writes:
/// date-string YYYY-MM-DD
///
/// The libraries time and chrono only handle +/- 10000 years.
pub fn serialise_date<Out>(bytes: &[u8], out: &mut Out) -> Result<(), CasErr>
where
    Out: Write,
{
    let days = i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    out.write(DOUBLE_QUOTE)?;
    write_date(days, out)?;
    out.write(DOUBLE_QUOTE)?;
    Ok(())
}

/// Given:
/// i64: microseconds after midnight
///
/// Writes:
/// time string HH:MM:SS, where seconds may have a fractional component
pub fn serialise_time_unzoned<Out>(bytes: &[u8], out: &mut Out) -> Result<(), CasErr>
where
    Out: Write,
{
    Ok(())
}

/// Given:
/// i64: microseconds after midnight
/// i32: offset seconds, seconds behind UTC. -01:00 is 3600 and +01:00 is -3600.
///
/// Writes:
/// time string HH:MM:SS+HH:MM(:SS), where the time-seconds may have a fractional component. The
/// offset seconds are optional, and never fractional. Postgres doesn’t distinguish between GMT and
/// UTC, so 0-offsets are always written +00:00, not as Z.
///
pub fn serialise_time_zoned<Out>(bytes: &[u8], out: &mut Out) -> Result<(), CasErr>
where
    Out: Write,
{
    Ok(())
}

/// Given:
/// i64: microseconds after 2000-01-01, negative numbers are microseconds before
///
/// Writes:
/// UTC datetime-string YYYY-MM-DDTHH:MM:SSZ.
///
/// TODO: is timestamptz different if I change the system clock of the db image?
pub fn serialise_datetime<Out>(bytes: &[u8], out: &mut Out) -> Result<(), CasErr>
where
    Out: Write,
{
    Ok(())
}

/// Given:
/// i64: microseconds, time within day
/// i32: days
/// i32: months
///
/// Writes:
/// ISO 8601 compact period string, e.g. "P3Y6M4DT12H30M5S". Empty interval is "P0D".
///
/// ISO 8601 doesn’t specify negative intervals, but I’m adding negative signs where applicable.
///
/// Postgres stores durations in this format because their length is not absolute, it’s a
/// description. For example, the year in  <date> + '1 year - 1 day' may be a leap year, if the
/// date is the year before one. The upshot is that I can reduce some parts, like months to years,
/// but not others, like months to days. Days may have more/fewer hours at daylight savings transitions.
pub fn serialise_duration<Out>(bytes: &[u8], out: &mut Out) -> Result<(), CasErr>
where
    Out: Write,
{
    Ok(())
}

// fn epoch_minus(days_in_past: i32) -> i32 {
//     let mut year = 2000;
//     let mut days = days_in_past;
//     let chunks = days / 1460970;
//     year -= chunks * 4000;
//     days -= chunks * 1460970;
//     while days >= -365 {
//         let length = if leap_year(year) { 366 } else { 365 };
//         year -= 1;
//         days -= length
//     }
//     eprintln!("year: {}, days: {}", year, days);
//     0
// }

/// Given the number of days to add to the Postgres epoch, calculate the date and print it. Does not
/// print the double quotes for the date string, because it may occur within a larger datetime string.
fn write_date<Out>(days_in_future: i32, out: &mut Out) -> Result<(), CasErr>
where
    Out: Write,
{
    let mut year = 2000;
    let mut days = days_in_future;
    // skip over some years when they’re really far in the future, should really benchmark this
    let chunks = days / 1460970;
    year += chunks * 4000;
    days -= chunks * 1460970;
    // While there are more days than a year, add/subtract a year and reduce the number of remaining days.
    if days.is_positive() {
        let mut length = if leap_year(year) { 366 } else { 365 };
        while days >= length {
            year += 1;
            days -= length;
            length = if leap_year(year) { 366 } else { 365 };
        }
    } else {
        let mut length = if leap_year(year - 1) { -366 } else { -365 };
        while days < 0 {
            year -= 1;
            days -= length;
            length = if leap_year(year - 1) { -366 } else { -365 };
        }
    }

    // Because the number of days is fixed and small, don’t bother iterating, just look it up.
    let month_day = if leap_year(year) {
        MONTH_DAYS_LEAP[days as usize]
    } else {
        MONTH_DAYS[days as usize]
    };

    // The Rust format strings and I don’t get along. The negative sign cuts into the padding, so
    // in order to preserve the correct width, I am printing the negative sign separately.
    let sign = if year > 0 { "" } else { "-" };
    write!(out, "{}{:04}-{}", sign, year.abs(), month_day)?;
    Ok(())
}

fn leap_year(year: i32) -> bool {
    // more clever way to do this?
    if year % 4 != 0 {
        false
    } else if year % 400 == 0 {
        true
    } else if year % 100 == 0 {
        false
    } else {
        true
    }
}

const MONTH_DAYS: [&str; 365] = [
    "01-01", "01-02", "01-03", "01-04", "01-05", "01-06", "01-07", "01-08", "01-09", "01-10",
    "01-11", "01-12", "01-13", "01-14", "01-15", "01-16", "01-17", "01-18", "01-19", "01-20",
    "01-21", "01-22", "01-23", "01-24", "01-25", "01-26", "01-27", "01-28", "01-29", "01-30",
    "01-31", "02-01", "02-02", "02-03", "02-04", "02-05", "02-06", "02-07", "02-08", "02-09",
    "02-10", "02-11", "02-12", "02-13", "02-14", "02-15", "02-16", "02-17", "02-18", "02-19",
    "02-20", "02-21", "02-22", "02-23", "02-24", "02-25", "02-26", "02-27", "02-28", "03-01",
    "03-02", "03-03", "03-04", "03-05", "03-06", "03-07", "03-08", "03-09", "03-10", "03-11",
    "03-12", "03-13", "03-14", "03-15", "03-16", "03-17", "03-18", "03-19", "03-20", "03-21",
    "03-22", "03-23", "03-24", "03-25", "03-26", "03-27", "03-28", "03-29", "03-30", "03-31",
    "04-01", "04-02", "04-03", "04-04", "04-05", "04-06", "04-07", "04-08", "04-09", "04-10",
    "04-11", "04-12", "04-13", "04-14", "04-15", "04-16", "04-17", "04-18", "04-19", "04-20",
    "04-21", "04-22", "04-23", "04-24", "04-25", "04-26", "04-27", "04-28", "04-29", "04-30",
    "05-01", "05-02", "05-03", "05-04", "05-05", "05-06", "05-07", "05-08", "05-09", "05-10",
    "05-11", "05-12", "05-13", "05-14", "05-15", "05-16", "05-17", "05-18", "05-19", "05-20",
    "05-21", "05-22", "05-23", "05-24", "05-25", "05-26", "05-27", "05-28", "05-29", "05-30",
    "05-31", "06-01", "06-02", "06-03", "06-04", "06-05", "06-06", "06-07", "06-08", "06-09",
    "06-10", "06-11", "06-12", "06-13", "06-14", "06-15", "06-16", "06-17", "06-18", "06-19",
    "06-20", "06-21", "06-22", "06-23", "06-24", "06-25", "06-26", "06-27", "06-28", "06-29",
    "06-30", "07-01", "07-02", "07-03", "07-04", "07-05", "07-06", "07-07", "07-08", "07-09",
    "07-10", "07-11", "07-12", "07-13", "07-14", "07-15", "07-16", "07-17", "07-18", "07-19",
    "07-20", "07-21", "07-22", "07-23", "07-24", "07-25", "07-26", "07-27", "07-28", "07-29",
    "07-30", "07-31", "08-01", "08-02", "08-03", "08-04", "08-05", "08-06", "08-07", "08-08",
    "08-09", "08-10", "08-11", "08-12", "08-13", "08-14", "08-15", "08-16", "08-17", "08-18",
    "08-19", "08-20", "08-21", "08-22", "08-23", "08-24", "08-25", "08-26", "08-27", "08-28",
    "08-29", "08-30", "08-31", "09-01", "09-02", "09-03", "09-04", "09-05", "09-06", "09-07",
    "09-08", "09-09", "09-10", "09-11", "09-12", "09-13", "09-14", "09-15", "09-16", "09-17",
    "09-18", "09-19", "09-20", "09-21", "09-22", "09-23", "09-24", "09-25", "09-26", "09-27",
    "09-28", "09-29", "09-30", "10-01", "10-02", "10-03", "10-04", "10-05", "10-06", "10-07",
    "10-08", "10-09", "10-10", "10-11", "10-12", "10-13", "10-14", "10-15", "10-16", "10-17",
    "10-18", "10-19", "10-20", "10-21", "10-22", "10-23", "10-24", "10-25", "10-26", "10-27",
    "10-28", "10-29", "10-30", "10-31", "11-01", "11-02", "11-03", "11-04", "11-05", "11-06",
    "11-07", "11-08", "11-09", "11-10", "11-11", "11-12", "11-13", "11-14", "11-15", "11-16",
    "11-17", "11-18", "11-19", "11-20", "11-21", "11-22", "11-23", "11-24", "11-25", "11-26",
    "11-27", "11-28", "11-29", "11-30", "12-01", "12-02", "12-03", "12-04", "12-05", "12-06",
    "12-07", "12-08", "12-09", "12-10", "12-11", "12-12", "12-13", "12-14", "12-15", "12-16",
    "12-17", "12-18", "12-19", "12-20", "12-21", "12-22", "12-23", "12-24", "12-25", "12-26",
    "12-27", "12-28", "12-29", "12-30", "12-31",
];
const MONTH_DAYS_LEAP: [&str; 366] = [
    "01-01", "01-02", "01-03", "01-04", "01-05", "01-06", "01-07", "01-08", "01-09", "01-10",
    "01-11", "01-12", "01-13", "01-14", "01-15", "01-16", "01-17", "01-18", "01-19", "01-20",
    "01-21", "01-22", "01-23", "01-24", "01-25", "01-26", "01-27", "01-28", "01-29", "01-30",
    "01-31", "02-01", "02-02", "02-03", "02-04", "02-05", "02-06", "02-07", "02-08", "02-09",
    "02-10", "02-11", "02-12", "02-13", "02-14", "02-15", "02-16", "02-17", "02-18", "02-19",
    "02-20", "02-21", "02-22", "02-23", "02-24", "02-25", "02-26", "02-27", "02-28", "02-29",
    "03-01", "03-02", "03-03", "03-04", "03-05", "03-06", "03-07", "03-08", "03-09", "03-10",
    "03-11", "03-12", "03-13", "03-14", "03-15", "03-16", "03-17", "03-18", "03-19", "03-20",
    "03-21", "03-22", "03-23", "03-24", "03-25", "03-26", "03-27", "03-28", "03-29", "03-30",
    "03-31", "04-01", "04-02", "04-03", "04-04", "04-05", "04-06", "04-07", "04-08", "04-09",
    "04-10", "04-11", "04-12", "04-13", "04-14", "04-15", "04-16", "04-17", "04-18", "04-19",
    "04-20", "04-21", "04-22", "04-23", "04-24", "04-25", "04-26", "04-27", "04-28", "04-29",
    "04-30", "05-01", "05-02", "05-03", "05-04", "05-05", "05-06", "05-07", "05-08", "05-09",
    "05-10", "05-11", "05-12", "05-13", "05-14", "05-15", "05-16", "05-17", "05-18", "05-19",
    "05-20", "05-21", "05-22", "05-23", "05-24", "05-25", "05-26", "05-27", "05-28", "05-29",
    "05-30", "05-31", "06-01", "06-02", "06-03", "06-04", "06-05", "06-06", "06-07", "06-08",
    "06-09", "06-10", "06-11", "06-12", "06-13", "06-14", "06-15", "06-16", "06-17", "06-18",
    "06-19", "06-20", "06-21", "06-22", "06-23", "06-24", "06-25", "06-26", "06-27", "06-28",
    "06-29", "06-30", "07-01", "07-02", "07-03", "07-04", "07-05", "07-06", "07-07", "07-08",
    "07-09", "07-10", "07-11", "07-12", "07-13", "07-14", "07-15", "07-16", "07-17", "07-18",
    "07-19", "07-20", "07-21", "07-22", "07-23", "07-24", "07-25", "07-26", "07-27", "07-28",
    "07-29", "07-30", "07-31", "08-01", "08-02", "08-03", "08-04", "08-05", "08-06", "08-07",
    "08-08", "08-09", "08-10", "08-11", "08-12", "08-13", "08-14", "08-15", "08-16", "08-17",
    "08-18", "08-19", "08-20", "08-21", "08-22", "08-23", "08-24", "08-25", "08-26", "08-27",
    "08-28", "08-29", "08-30", "08-31", "09-01", "09-02", "09-03", "09-04", "09-05", "09-06",
    "09-07", "09-08", "09-09", "09-10", "09-11", "09-12", "09-13", "09-14", "09-15", "09-16",
    "09-17", "09-18", "09-19", "09-20", "09-21", "09-22", "09-23", "09-24", "09-25", "09-26",
    "09-27", "09-28", "09-29", "09-30", "10-01", "10-02", "10-03", "10-04", "10-05", "10-06",
    "10-07", "10-08", "10-09", "10-10", "10-11", "10-12", "10-13", "10-14", "10-15", "10-16",
    "10-17", "10-18", "10-19", "10-20", "10-21", "10-22", "10-23", "10-24", "10-25", "10-26",
    "10-27", "10-28", "10-29", "10-30", "10-31", "11-01", "11-02", "11-03", "11-04", "11-05",
    "11-06", "11-07", "11-08", "11-09", "11-10", "11-11", "11-12", "11-13", "11-14", "11-15",
    "11-16", "11-17", "11-18", "11-19", "11-20", "11-21", "11-22", "11-23", "11-24", "11-25",
    "11-26", "11-27", "11-28", "11-29", "11-30", "12-01", "12-02", "12-03", "12-04", "12-05",
    "12-06", "12-07", "12-08", "12-09", "12-10", "12-11", "12-12", "12-13", "12-14", "12-15",
    "12-16", "12-17", "12-18", "12-19", "12-20", "12-21", "12-22", "12-23", "12-24", "12-25",
    "12-26", "12-27", "12-28", "12-29", "12-30", "12-31",
];
