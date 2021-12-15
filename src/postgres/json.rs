// fn write_value<Out>(bytes: &[u8], parser: &Parser, out: &mut Out) -> Result<(), CasErr>
// where
//     Out: Write,
// {
//     match parser {
//         Parser::Date => {
//             // Dates are stored as an i32, representing days after 2000-01-01. The Rust time libraries
//             // only handle +/- 10000 years. Calculating future dates is easy enough, but I’m not as sure
//             // about historical dates. Since Postgres only goes back to 4713 BC, I can use the time
//             // crate for historical dates.
//             let days: i64 = i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as i64;
//             if days < 1 {
//                 let epoch: time::Date =
//                     time::Date::from_calendar_date(2000, time::Month::January, 1)
//                         .expect("Epoch is always valid date.");
//                 let then = epoch.add(time::Duration::days(days));
//                 out.write(DOUBLE_QUOTE)?;
//                 out.write(then.to_string().as_bytes())?;
//                 out.write(DOUBLE_QUOTE)?;
//             } else {
//                 out.write(DOUBLE_QUOTE)?;
//                 out.write(post_epoch(days).as_bytes())?;
//                 out.write(DOUBLE_QUOTE)?;
//             }
//         }
//         Parser::Interval => {
//             // ISO 8601 doesn’t specify negative intervals, so this is a best effort for now. I
//             // can’t simplify all of the quantities because '1 year - 1 day' will be different
//             // depending on what starting date it is applied to. I can do some though, all years
//             // will have 12 months.
//             let mut interval = BinaryReader::from(bytes, ByteOrder::BigEndian);
//             let mut microseconds = interval.i64();
//             let days = interval.i32();
//             let mut months = interval.i32();
//
//             if days == 0 && months == 0 && microseconds == 0 {
//                 out.write("\"P0D\"".as_bytes())?;
//                 return Ok(());
//             }
//
//             let years = months / 12;
//             months -= years * 12;
//             let hour_us = 3600000000;
//             let minute_us = 60000000;
//             let second_us = 1000000.0;
//             let hours = microseconds / hour_us;
//             microseconds -= hours * hour_us;
//             let minutes = microseconds / minute_us;
//             microseconds -= minutes * minute_us;
//             let seconds = (microseconds as f32) / second_us;
//
//             let mut duration_str = "\"P".to_owned();
//             if years != 0 {
//                 duration_str.push_str(&years.to_string());
//                 duration_str.push('Y');
//             };
//             if months != 0 {
//                 duration_str.push_str(&months.to_string());
//                 duration_str.push('M');
//             };
//             if days != 0 {
//                 duration_str.push_str(&days.to_string());
//                 duration_str.push('D');
//             };
//             if hours != 0 || minutes != 0 || seconds != 0.0 {
//                 duration_str.push('T');
//             }
//             if hours != 0 {
//                 duration_str.push_str(&hours.to_string());
//                 duration_str.push('H');
//             };
//             if minutes != 0 {
//                 duration_str.push_str(&minutes.to_string());
//                 duration_str.push('M');
//             };
//             if seconds != 0.0 {
//                 duration_str.push_str(&seconds.to_string());
//                 duration_str.push('S');
//             };
//             duration_str.push('"');
//             out.write(duration_str.as_bytes())?;
//         }
//         Parser::String => {
//             serde_json::to_writer(out, std::str::from_utf8(bytes)?)?;
//         }
//         Parser::EWKB => {
//             let geom = ewkb::parse_geom(bytes);
//             serde_json::to_writer(out, &geom)?;
//         }
//         Parser::Tid => {
//             // ( i32, i16 )
//             let block = i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
//             let offset = i16::from_be_bytes([bytes[4], bytes[5]]);
//             out.write(LEFT_SQUARE)?;
//             serde_json::to_writer(&mut (*out), &block)?;
//             out.write(COMMA)?;
//             serde_json::to_writer(&mut (*out), &offset)?;
//             out.write(RIGHT_SQUARE)?;
//         }
//         Parser::Timestamp => {
//             let mut microseconds = i64::from_be_bytes([
//                 bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
//             ]);
//             // working
//             // let epoch = time::OffsetDateTime::from_unix_timestamp(946684800).unwrap();
//             // eprintln!("{}", epoch.to_string());
//             // let then = epoch.add(time::Duration::microseconds(microseconds));
//             // eprintln!("{:?}", then);
//
//             // experimental, also working
//             /*
//             TODO: tidy this up
//             - make module out of this file
//             - separate file for time stuff
//             - separate out time calculations from printing
//             so timestamp is just microseconds after 2000-01-01
//             // I'm curious if how it works in historical time with shifts and whatnot
//             */
//             // let hour_us = 3600000000;
//             // let days = microseconds / (24 * hour_us);
//             // microseconds -= (days * 24 * hour_us);
//             // eprintln!("days: {}", post_epoch(days));
//             //
//             // let minute_us = 60000000;
//             // let second_us = 1000000.0;
//             // let hours = microseconds / hour_us;
//             // microseconds -= hours * hour_us;
//             // let minutes = microseconds / minute_us;
//             // microseconds -= minutes * minute_us;
//             // let seconds = (microseconds as f64) / second_us;
//             // // This is a ridiculous hack, but I cannot figure out how to pad just the integral part
//             // // of a floating point number, and I want to avoid an additional string allocation.
//             // let padding = if seconds < 10.0 { "0" } else { "" };
//             // eprintln!("\"{:02}:{:02}:{}{}\"", hours, minutes, padding, seconds);
//         }
//         Parser::TimeUnzoned => {
//             let mut microseconds = i64::from_be_bytes([
//                 bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
//             ]);
//             let hour_us = 3600000000;
//             let minute_us = 60000000;
//             let second_us = 1000000.0;
//             let hours = microseconds / hour_us;
//             microseconds -= hours * hour_us;
//             let minutes = microseconds / minute_us;
//             microseconds -= minutes * minute_us;
//             let seconds = (microseconds as f64) / second_us;
//             // This is a ridiculous hack, but I cannot figure out how to pad just the integral part
//             // of a floating point number, and I want to avoid an additional string allocation.
//             let padding = if seconds < 10.0 { "0" } else { "" };
//             write!(
//                 out,
//                 "\"{:02}:{:02}:{}{}\"",
//                 hours, minutes, padding, seconds
//             )?;
//         }
//         Parser::TimeZoned => {
//             // TODO: repeats the above, should be factored out.
//             let mut microseconds = i64::from_be_bytes([
//                 bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
//             ]);
//             let hour_us = 3600000000;
//             let minute_us = 60000000;
//             let second_us = 1000000.0;
//
//             let hours = microseconds / hour_us;
//             microseconds -= hours * hour_us;
//             let minutes = microseconds / minute_us;
//             microseconds -= minutes * minute_us;
//             let seconds = (microseconds as f32) / second_us;
//             // This is a ridiculous hack, but I cannot figure out how to pad just the integral part
//             // of a floating point number, and I want to avoid an additional string allocation.
//             let padding = if seconds < 10.0 { "0" } else { "" };
//             write!(out, "\"{:02}:{:02}:{}{}", hours, minutes, padding, seconds)?;
//             // end repetition
//
//             let mut offset_seconds =
//                 0 - i32::from_be_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);
//             let offset_hours = offset_seconds / 3600;
//             offset_seconds -= offset_hours * 3600;
//             let offset_minutes = offset_seconds / 60;
//             offset_seconds -= offset_minutes * 60;
//             if offset_seconds > 0 {
//                 write!(
//                     out,
//                     "{:+03}:{:02}:{:02}\"",
//                     offset_hours, offset_minutes, offset_seconds
//                 )
//             } else {
//                 write!(out, "{:+03}:{:02}\"", offset_hours, offset_minutes)
//             }?;
//         }
//         Parser::Unknown => {
//             serde_json::to_writer(out, "???")?;
//         }
//     }
//     Ok(())
// }
// fn post_epoch(days_in_future: i64) -> String {
//     let mut year = 2000;
//     let mut days = days_in_future;
//     let chunks = days / 1460970;
//     year += chunks * 4000;
//     days -= chunks * 1460970;
//     while days >= 365 {
//         let length = if leap_year(year) { 366 } else { 365 };
//         year += 1;
//         days -= length
//     }
//     let month_day = if leap_year(year) {
//         MONTH_DAYS_LEAP[days as usize]
//     } else {
//         MONTH_DAYS[days as usize]
//     };
//     format!("{}-{}", year, month_day)
// }
//
// fn leap_year(year: i64) -> bool {
//     // more clever way to do this?
//     if year % 4 != 0 {
//         false
//     } else if year % 400 == 0 {
//         true
//     } else if year % 100 == 0 {
//         false
//     } else {
//         true
//     }
// }
//
// const MONTH_DAYS: [&str; 365] = [
//     "01-01", "01-02", "01-03", "01-04", "01-05", "01-06", "01-07", "01-08", "01-09", "01-10",
//     "01-11", "01-12", "01-13", "01-14", "01-15", "01-16", "01-17", "01-18", "01-19", "01-20",
//     "01-21", "01-22", "01-23", "01-24", "01-25", "01-26", "01-27", "01-28", "01-29", "01-30",
//     "01-31", "02-01", "02-02", "02-03", "02-04", "02-05", "02-06", "02-07", "02-08", "02-09",
//     "02-10", "02-11", "02-12", "02-13", "02-14", "02-15", "02-16", "02-17", "02-18", "02-19",
//     "02-20", "02-21", "02-22", "02-23", "02-24", "02-25", "02-26", "02-27", "02-28", "03-01",
//     "03-02", "03-03", "03-04", "03-05", "03-06", "03-07", "03-08", "03-09", "03-10", "03-11",
//     "03-12", "03-13", "03-14", "03-15", "03-16", "03-17", "03-18", "03-19", "03-20", "03-21",
//     "03-22", "03-23", "03-24", "03-25", "03-26", "03-27", "03-28", "03-29", "03-30", "03-31",
//     "04-01", "04-02", "04-03", "04-04", "04-05", "04-06", "04-07", "04-08", "04-09", "04-10",
//     "04-11", "04-12", "04-13", "04-14", "04-15", "04-16", "04-17", "04-18", "04-19", "04-20",
//     "04-21", "04-22", "04-23", "04-24", "04-25", "04-26", "04-27", "04-28", "04-29", "04-30",
//     "05-01", "05-02", "05-03", "05-04", "05-05", "05-06", "05-07", "05-08", "05-09", "05-10",
//     "05-11", "05-12", "05-13", "05-14", "05-15", "05-16", "05-17", "05-18", "05-19", "05-20",
//     "05-21", "05-22", "05-23", "05-24", "05-25", "05-26", "05-27", "05-28", "05-29", "05-30",
//     "05-31", "06-01", "06-02", "06-03", "06-04", "06-05", "06-06", "06-07", "06-08", "06-09",
//     "06-10", "06-11", "06-12", "06-13", "06-14", "06-15", "06-16", "06-17", "06-18", "06-19",
//     "06-20", "06-21", "06-22", "06-23", "06-24", "06-25", "06-26", "06-27", "06-28", "06-29",
//     "06-30", "07-01", "07-02", "07-03", "07-04", "07-05", "07-06", "07-07", "07-08", "07-09",
//     "07-10", "07-11", "07-12", "07-13", "07-14", "07-15", "07-16", "07-17", "07-18", "07-19",
//     "07-20", "07-21", "07-22", "07-23", "07-24", "07-25", "07-26", "07-27", "07-28", "07-29",
//     "07-30", "07-31", "08-01", "08-02", "08-03", "08-04", "08-05", "08-06", "08-07", "08-08",
//     "08-09", "08-10", "08-11", "08-12", "08-13", "08-14", "08-15", "08-16", "08-17", "08-18",
//     "08-19", "08-20", "08-21", "08-22", "08-23", "08-24", "08-25", "08-26", "08-27", "08-28",
//     "08-29", "08-30", "08-31", "09-01", "09-02", "09-03", "09-04", "09-05", "09-06", "09-07",
//     "09-08", "09-09", "09-10", "09-11", "09-12", "09-13", "09-14", "09-15", "09-16", "09-17",
//     "09-18", "09-19", "09-20", "09-21", "09-22", "09-23", "09-24", "09-25", "09-26", "09-27",
//     "09-28", "09-29", "09-30", "10-01", "10-02", "10-03", "10-04", "10-05", "10-06", "10-07",
//     "10-08", "10-09", "10-10", "10-11", "10-12", "10-13", "10-14", "10-15", "10-16", "10-17",
//     "10-18", "10-19", "10-20", "10-21", "10-22", "10-23", "10-24", "10-25", "10-26", "10-27",
//     "10-28", "10-29", "10-30", "10-31", "11-01", "11-02", "11-03", "11-04", "11-05", "11-06",
//     "11-07", "11-08", "11-09", "11-10", "11-11", "11-12", "11-13", "11-14", "11-15", "11-16",
//     "11-17", "11-18", "11-19", "11-20", "11-21", "11-22", "11-23", "11-24", "11-25", "11-26",
//     "11-27", "11-28", "11-29", "11-30", "12-01", "12-02", "12-03", "12-04", "12-05", "12-06",
//     "12-07", "12-08", "12-09", "12-10", "12-11", "12-12", "12-13", "12-14", "12-15", "12-16",
//     "12-17", "12-18", "12-19", "12-20", "12-21", "12-22", "12-23", "12-24", "12-25", "12-26",
//     "12-27", "12-28", "12-29", "12-30", "12-31",
// ];
// const MONTH_DAYS_LEAP: [&str; 366] = [
//     "01-01", "01-02", "01-03", "01-04", "01-05", "01-06", "01-07", "01-08", "01-09", "01-10",
//     "01-11", "01-12", "01-13", "01-14", "01-15", "01-16", "01-17", "01-18", "01-19", "01-20",
//     "01-21", "01-22", "01-23", "01-24", "01-25", "01-26", "01-27", "01-28", "01-29", "01-30",
//     "01-31", "02-01", "02-02", "02-03", "02-04", "02-05", "02-06", "02-07", "02-08", "02-09",
//     "02-10", "02-11", "02-12", "02-13", "02-14", "02-15", "02-16", "02-17", "02-18", "02-19",
//     "02-20", "02-21", "02-22", "02-23", "02-24", "02-25", "02-26", "02-27", "02-28", "02-29",
//     "03-01", "03-02", "03-03", "03-04", "03-05", "03-06", "03-07", "03-08", "03-09", "03-10",
//     "03-11", "03-12", "03-13", "03-14", "03-15", "03-16", "03-17", "03-18", "03-19", "03-20",
//     "03-21", "03-22", "03-23", "03-24", "03-25", "03-26", "03-27", "03-28", "03-29", "03-30",
//     "03-31", "04-01", "04-02", "04-03", "04-04", "04-05", "04-06", "04-07", "04-08", "04-09",
//     "04-10", "04-11", "04-12", "04-13", "04-14", "04-15", "04-16", "04-17", "04-18", "04-19",
//     "04-20", "04-21", "04-22", "04-23", "04-24", "04-25", "04-26", "04-27", "04-28", "04-29",
//     "04-30", "05-01", "05-02", "05-03", "05-04", "05-05", "05-06", "05-07", "05-08", "05-09",
//     "05-10", "05-11", "05-12", "05-13", "05-14", "05-15", "05-16", "05-17", "05-18", "05-19",
//     "05-20", "05-21", "05-22", "05-23", "05-24", "05-25", "05-26", "05-27", "05-28", "05-29",
//     "05-30", "05-31", "06-01", "06-02", "06-03", "06-04", "06-05", "06-06", "06-07", "06-08",
//     "06-09", "06-10", "06-11", "06-12", "06-13", "06-14", "06-15", "06-16", "06-17", "06-18",
//     "06-19", "06-20", "06-21", "06-22", "06-23", "06-24", "06-25", "06-26", "06-27", "06-28",
//     "06-29", "06-30", "07-01", "07-02", "07-03", "07-04", "07-05", "07-06", "07-07", "07-08",
//     "07-09", "07-10", "07-11", "07-12", "07-13", "07-14", "07-15", "07-16", "07-17", "07-18",
//     "07-19", "07-20", "07-21", "07-22", "07-23", "07-24", "07-25", "07-26", "07-27", "07-28",
//     "07-29", "07-30", "07-31", "08-01", "08-02", "08-03", "08-04", "08-05", "08-06", "08-07",
//     "08-08", "08-09", "08-10", "08-11", "08-12", "08-13", "08-14", "08-15", "08-16", "08-17",
//     "08-18", "08-19", "08-20", "08-21", "08-22", "08-23", "08-24", "08-25", "08-26", "08-27",
//     "08-28", "08-29", "08-30", "08-31", "09-01", "09-02", "09-03", "09-04", "09-05", "09-06",
//     "09-07", "09-08", "09-09", "09-10", "09-11", "09-12", "09-13", "09-14", "09-15", "09-16",
//     "09-17", "09-18", "09-19", "09-20", "09-21", "09-22", "09-23", "09-24", "09-25", "09-26",
//     "09-27", "09-28", "09-29", "09-30", "10-01", "10-02", "10-03", "10-04", "10-05", "10-06",
//     "10-07", "10-08", "10-09", "10-10", "10-11", "10-12", "10-13", "10-14", "10-15", "10-16",
//     "10-17", "10-18", "10-19", "10-20", "10-21", "10-22", "10-23", "10-24", "10-25", "10-26",
//     "10-27", "10-28", "10-29", "10-30", "10-31", "11-01", "11-02", "11-03", "11-04", "11-05",
//     "11-06", "11-07", "11-08", "11-09", "11-10", "11-11", "11-12", "11-13", "11-14", "11-15",
//     "11-16", "11-17", "11-18", "11-19", "11-20", "11-21", "11-22", "11-23", "11-24", "11-25",
//     "11-26", "11-27", "11-28", "11-29", "11-30", "12-01", "12-02", "12-03", "12-04", "12-05",
//     "12-06", "12-07", "12-08", "12-09", "12-10", "12-11", "12-12", "12-13", "12-14", "12-15",
//     "12-16", "12-17", "12-18", "12-19", "12-20", "12-21", "12-22", "12-23", "12-24", "12-25",
//     "12-26", "12-27", "12-28", "12-29", "12-30", "12-31",
// ];
