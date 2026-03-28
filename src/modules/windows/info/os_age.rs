use std::env;
use std::fmt::Write;
use std::fs;
use std::path::PathBuf;
use std::ptr::null_mut;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::modules::enums::OsAgeShorthand;
use winapi::shared::minwindef::DWORD;
use winapi::shared::winerror::ERROR_SUCCESS;
use winapi::um::winreg::{RegGetValueW, HKEY_LOCAL_MACHINE, RRF_RT_REG_DWORD, RRF_RT_REG_QWORD};

/// Returns the OS "age" (time since Windows installation) formatted per shorthand.
pub fn get_os_age(shorthand: OsAgeShorthand) -> Option<String> {
    let install_epoch = read_install_epoch_seconds()?;
    let now = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs();

    // Guard against clock skew or unknown/invalid install time.
    let seconds = now.saturating_sub(install_epoch);

    let days = seconds / 86_400;
    let hours = (seconds / 3_600) % 24;
    let minutes = (seconds / 60) % 60;

    let mut buf = String::with_capacity(32);

    match shorthand {
        OsAgeShorthand::Full => {
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
        OsAgeShorthand::Tiny => {
            if days > 0 {
                write!(buf, "{} days", days).ok()?;
            }
        }
        OsAgeShorthand::Seconds => {
            write!(buf, "{}s", seconds).ok()?;
        }
    }

    Some(buf.trim_end_matches([' ', ','].as_ref()).to_string())
}

/// Best-effort detection of install time (epoch seconds) for Windows.
/// Strategy:
/// 1) Read `InstallDate` from the registry (seconds since UNIX epoch).
/// 2) Fallback to `InstallTimeStamp` (FILETIME) if present.
/// 3) Fallback to the creation timestamp of `%SystemRoot%`.
fn read_install_epoch_seconds() -> Option<u64> {
    read_registry_install_date()
        .or_else(read_registry_install_timestamp)
        .or_else(read_windows_dir_creation)
}

fn read_registry_install_date() -> Option<u64> {
    let key_path = to_wide("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion");
    let value_name = to_wide("InstallDate");

    let mut data: DWORD = 0;
    let mut data_size = std::mem::size_of::<DWORD>() as u32;

    let status = unsafe {
        RegGetValueW(
            HKEY_LOCAL_MACHINE,
            key_path.as_ptr(),
            value_name.as_ptr(),
            RRF_RT_REG_DWORD,
            null_mut(),
            &mut data as *mut _ as *mut _,
            &mut data_size,
        )
    };

    if status == ERROR_SUCCESS as i32 {
        let value = u64::from(data);
        if value > 0 {
            return Some(value);
        }
    }

    None
}

fn read_registry_install_timestamp() -> Option<u64> {
    let key_path = to_wide("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion");
    let value_name = to_wide("InstallTimeStamp");

    let mut data: u64 = 0;
    let mut data_size = std::mem::size_of::<u64>() as u32;

    let status = unsafe {
        RegGetValueW(
            HKEY_LOCAL_MACHINE,
            key_path.as_ptr(),
            value_name.as_ptr(),
            RRF_RT_REG_QWORD,
            null_mut(),
            &mut data as *mut _ as *mut _,
            &mut data_size,
        )
    };

    if status == ERROR_SUCCESS as i32 && data > 0 {
        return filetime_to_unix_epoch(data);
    }

    None
}

fn read_windows_dir_creation() -> Option<u64> {
    let system_root = env::var_os("SystemRoot")
        .or_else(|| env::var_os("WINDIR"))
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(r"C:\Windows"));

    let metadata = fs::metadata(system_root).ok()?;
    let created = metadata.created().ok()?;
    let duration = created.duration_since(UNIX_EPOCH).ok()?;
    let secs = duration.as_secs();
    if secs > 0 {
        Some(secs)
    } else {
        None
    }
}

fn filetime_to_unix_epoch(filetime: u64) -> Option<u64> {
    const HUNDREDS_OF_NS_PER_SECOND: u64 = 10_000_000;
    const SECONDS_BETWEEN_EPOCHS: u64 = 11_644_473_600;

    let seconds = filetime / HUNDREDS_OF_NS_PER_SECOND;
    seconds.checked_sub(SECONDS_BETWEEN_EPOCHS)
}

fn to_wide(s: &str) -> Vec<u16> {
    use std::os::windows::ffi::OsStrExt;
    std::ffi::OsStr::new(s)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}
