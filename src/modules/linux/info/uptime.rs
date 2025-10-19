use std::fmt::Write;
use std::fs;

use crate::modules::enums::UptimeShorthand;

pub fn get_uptime(shorthand: UptimeShorthand) -> Option<String> {
    let seconds = read_uptime_seconds()?;

    Some(format_uptime(seconds, shorthand))
}

fn read_uptime_seconds() -> Option<u64> {
    let contents = fs::read_to_string("/proc/uptime").ok()?;
    let end = contents.find(' ')?; // Faster than split
    let raw = contents.get(0..end)?;
    let secs = raw.parse::<f64>().ok()?;
    Some(secs.floor() as u64)
}

fn format_uptime(seconds: u64, shorthand: UptimeShorthand) -> String {
    let days = seconds / 86400;
    let hours = (seconds / 3600) % 24;
    let minutes = (seconds / 60) % 60;

    let mut buf = String::with_capacity(32);

    match shorthand {
        UptimeShorthand::Full => {
            if days > 0 {
                let _ = write!(buf, "{} day{}, ", days, if days != 1 { "s" } else { "" });
            }
            if hours > 0 {
                let _ = write!(buf, "{} hour{}, ", hours, if hours != 1 { "s" } else { "" });
            }
            if minutes > 0 {
                let _ = write!(
                    buf,
                    "{} minute{}",
                    minutes,
                    if minutes != 1 { "s" } else { "" }
                );
            }
            if buf.is_empty() {
                let _ = write!(buf, "{} seconds", seconds);
            }
        }
        UptimeShorthand::Tiny => {
            if days > 0 {
                let _ = write!(buf, "{}d", days);
            }
            if hours > 0 {
                if !buf.is_empty() {
                    buf.push(' ');
                }
                let _ = write!(buf, "{}h", hours);
            }
            if minutes > 0 {
                if !buf.is_empty() {
                    buf.push(' ');
                }
                let _ = write!(buf, "{}m", minutes);
            }
            if buf.is_empty() {
                let _ = write!(buf, "{}s", seconds);
            }
        }
        UptimeShorthand::Seconds => {
            let _ = write!(buf, "{}s", seconds);
        }
    }

    buf.trim_end_matches([' ', ','].as_ref()).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_full_uptime() {
        let result = format_uptime(86_400 + 3_600 + 120, UptimeShorthand::Full);
        assert!(
            result.contains("1 day") && result.contains("1 hour") && result.contains("2 minutes"),
            "unexpected format: {result}"
        );
    }

    #[test]
    fn formats_tiny_uptime() {
        let result = format_uptime(86_400 * 3 + 3_600 * 2 + 120, UptimeShorthand::Tiny);
        assert!(result.contains("3d"));
        assert!(result.contains("2h"));
        assert!(result.contains("2m"));
    }

    #[test]
    fn formats_seconds_uptime() {
        let result = format_uptime(15, UptimeShorthand::Seconds);
        assert_eq!(result, "15s");
    }
}
