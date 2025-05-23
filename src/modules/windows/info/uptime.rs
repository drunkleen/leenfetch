use std::fmt::Write;

use crate::modules::enums::UptimeShorthand;

#[inline(always)]
pub fn get_uptime(shorthand: UptimeShorthand) -> Option<String> {
    let seconds = read_uptime_seconds();

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

fn read_uptime_seconds() -> u64 {
    unsafe {
        // Use Windows API: GetTickCount64 returns milliseconds
        let millis = GetTickCount64();
        (millis / 1000) as u64
    }
}

#[link(name = "kernel32")]
extern "system" {
    fn GetTickCount64() -> u64;
}
