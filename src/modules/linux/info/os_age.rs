use std::fmt::Write;
use std::fs;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::modules::enums::OsAgeShorthand;

/// Returns the OS "age" (time since root FS creation/install) formatted per shorthand.
/// Mirrors the style of your `get_uptime` function.
pub fn get_os_age(shorthand: OsAgeShorthand) -> Option<String> {
    let install_epoch = read_install_epoch_seconds()?;
    let now = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs();

    // Guard against clock skew or unknown/invalid install time
    let seconds = now.saturating_sub(install_epoch);

    Some(format_age(seconds, shorthand))
}

/// Best-effort detection of install time (epoch seconds) of the root filesystem.
/// Strategy:
/// 1) Try `metadata("/").created()` (when supported by platform/fs)
/// 2) Fallback to `stat -c %W /` (Linux) which returns birth time or 0/-1 if unknown
fn read_install_epoch_seconds() -> Option<u64> {
    // Try std first (portable when supported)
    if let Ok(md) = fs::metadata("/") {
        if let Ok(created) = md.created() {
            if let Ok(dur) = created.duration_since(UNIX_EPOCH) {
                let secs = dur.as_secs();
                if secs > 0 {
                    return Some(secs);
                }
            }
        }
    }

    // Fallback: Linux `stat` birth time for `/`
    let out = Command::new("stat").args(["-c", "%W", "/"]).output().ok()?;

    if !out.status.success() {
        return None;
    }

    let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
    // %W might be "0" or "-1" if unknown
    if s == "0" || s == "-1" {
        return None;
    }

    // Handle potential negative parse safely
    if let Ok(v) = s.parse::<i64>() {
        if v > 0 {
            return Some(v as u64);
        }
    }

    None
}

fn format_age(seconds: u64, shorthand: OsAgeShorthand) -> String {
    let days = seconds / 86_400;
    let hours = (seconds / 3_600) % 24;
    let minutes = (seconds / 60) % 60;

    let mut buf = String::with_capacity(32);

    match shorthand {
        OsAgeShorthand::Full => {
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
        OsAgeShorthand::Tiny => {
            if days > 0 {
                let _ = write!(buf, "{} days", days);
            }
        }
        OsAgeShorthand::Seconds => {
            let _ = write!(buf, "{}s", seconds);
        }
    }

    buf.trim_end_matches([' ', ','].as_ref()).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_full_age() {
        let result = format_age(86_400 * 2 + 3_600 + 120, OsAgeShorthand::Full);
        assert!(
            result.contains("2 days") && result.contains("1 hour") && result.contains("2 minutes"),
            "unexpected format: {result}"
        );
    }

    #[test]
    fn formats_tiny_age() {
        let result = format_age(86_400 * 5 + 300, OsAgeShorthand::Tiny);
        assert_eq!(result, "5 days");
    }

    #[test]
    fn formats_seconds_age() {
        let result = format_age(42, OsAgeShorthand::Seconds);
        assert_eq!(result, "42s");
    }
}
