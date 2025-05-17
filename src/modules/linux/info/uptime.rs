use std::fmt::Write;
use std::fs;

use crate::modules::enums::UptimeShorthand;

pub fn get_uptime(shorthand: UptimeShorthand) -> Option<String> {
    let seconds = read_uptime_seconds()?;

    let days = seconds / 86400;
    let hours = (seconds / 3600) % 24;
    let minutes = (seconds / 60) % 60;

    let mut buf = String::with_capacity(32);

    match shorthand {
        UptimeShorthand::Full => {
            if days > 0 {
                write!(buf, "{} day{}, ", days, if days != 1 { "s" } else { "" }).ok()?;
            }
            if hours > 0 {
                write!(buf, "{} hour{}, ", hours, if hours != 1 { "s" } else { "" }).ok()?;
            }
            if minutes > 0 {
                write!(
                    buf,
                    "{} minute{}",
                    minutes,
                    if minutes != 1 { "s" } else { "" }
                )
                .ok()?;
            }
            if buf.is_empty() {
                write!(buf, "{} seconds", seconds).ok()?;
            }
        }
        UptimeShorthand::Tiny => {
            if days > 0 {
                write!(buf, "{}d", days).ok()?;
            }
            if hours > 0 {
                if !buf.is_empty() {
                    buf.push(' ');
                }
                write!(buf, "{}h", hours).ok()?;
            }
            if minutes > 0 {
                if !buf.is_empty() {
                    buf.push(' ');
                }
                write!(buf, "{}m", minutes).ok()?;
            }
            if buf.is_empty() {
                write!(buf, "{}s", seconds).ok()?;
            }
        }
        UptimeShorthand::Seconds => {
            write!(buf, "{}s", seconds).ok()?;
        }
    }

    Some(buf.trim_end_matches([' ', ','].as_ref()).to_string())
}

fn read_uptime_seconds() -> Option<u64> {
    let contents = fs::read_to_string("/proc/uptime").ok()?;
    let end = contents.find(' ')?; // Faster than split
    let raw = contents.get(0..end)?;
    let secs = raw.parse::<f64>().ok()?;
    Some(secs.floor() as u64)
}
