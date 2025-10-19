use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;
use std::process::Command;
use winapi::shared::minwindef::DWORD;
use winapi::um::winver::{VerQueryValueW, GetFileVersionInfoW, GetFileVersionInfoSizeW};

/// Desktop Environment enum
pub fn get_de(show_version: bool, wm: Option<&str>) -> Option<String> {
    let tasklist = Command::new("tasklist").output().ok()?;
    let list = String::from_utf8_lossy(&tasklist.stdout).to_lowercase();

    for &entry in KNOWN_DES {
        if list.contains(entry) {
            let mut de = normalize_de(entry);

            // Avoid WM = DE collision
            if let Some(wm_name) = wm {
                if de.eq_ignore_ascii_case(wm_name) {
                    return None;
                }
            }

            // Optional version tag
            if show_version {
                if let Some(ver) = get_shell_version(entry) {
                    de.push(' ');
                    de.push_str(&ver);
                }
            }

            return Some(de);
        }
    }

    None
}

fn normalize_de(proc: &str) -> String {
    match proc {
        "explorer.exe" => "Explorer".to_string(),
        "cairodesktop.exe" => "Cairo".to_string(),
        "bblean.exe" => "BBLean".to_string(),
        "e17.exe" => "Enlightenment".to_string(),
        "xoblite.exe" => "xoblite".to_string(),
        _ => proc.trim_end_matches(".exe").to_string(),
    }
}

fn get_shell_version(shell_exe: &str) -> Option<String> {
    // Try to read version from the file in System32. This avoids spawning WMIC.
    let path = format!("C:\\Windows\\System32\\{}", shell_exe);
    let wide: Vec<u16> = OsStr::new(&path).encode_wide().chain(Some(0)).collect();
    unsafe {
        let mut handle: DWORD = 0;
        let size = GetFileVersionInfoSizeW(wide.as_ptr(), &mut handle);
        if size == 0 { return None; }

        let mut buf: Vec<u8> = vec![0u8; size as usize];
        if GetFileVersionInfoW(wide.as_ptr(), 0, size, buf.as_mut_ptr() as *mut _) == 0 {
            return None;
        }

        // Determine language/codepage from Translation block
        let mut trans_ptr: *mut winapi::ctypes::c_void = null_mut();
        let mut trans_len: u32 = 0;
        let trans_key: Vec<u16> = OsStr::new("\\VarFileInfo\\Translation")
            .encode_wide()
            .chain(Some(0))
            .collect();
        if VerQueryValueW(
            buf.as_mut_ptr() as *mut _,
            trans_key.as_ptr(),
            &mut trans_ptr,
            &mut trans_len as *mut _,
        ) == 0
            || trans_ptr.is_null()
            || trans_len < 4
        {
            return None;
        }

        let trans = trans_ptr as *const u16;
        let lang = *trans as u32;
        let codepage = *trans.add(1) as u32;

        // Query the FileVersion string for this language/codepage
        let key = format!("\\StringFileInfo\\{:04x}{:04x}\\FileVersion", lang, codepage);
        let key_w: Vec<u16> = OsStr::new(&key).encode_wide().chain(Some(0)).collect();
        let mut str_ptr: *mut winapi::ctypes::c_void = null_mut();
        let mut str_len: u32 = 0;
        if VerQueryValueW(
            buf.as_mut_ptr() as *mut _,
            key_w.as_ptr(),
            &mut str_ptr,
            &mut str_len as *mut _,
        ) == 0
            || str_ptr.is_null()
        {
            return None;
        }

        let p = str_ptr as *const u16;
        let mut end = 0usize;
        while *p.add(end) != 0 {
            end += 1;
        }
        let slice = std::slice::from_raw_parts(p, end);
        Some(String::from_utf16_lossy(slice))
    }
}

/// Known alternative or default Windows desktop environments / shells
const KNOWN_DES: &[&str] = &[
    "explorer.exe",     // Default shell
    "cairodesktop.exe", // Cairo Shell
    "bblean.exe",       // BlackBox for Windows
    "xoblite.exe",      // xoblite Blackbox fork
    "e17.exe",          // Enlightenment on Windows
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_known_shells() {
        assert_eq!(normalize_de("explorer.exe"), "Explorer");
        assert_eq!(normalize_de("cairodesktop.exe"), "Cairo");
        assert_eq!(normalize_de("xoblite.exe"), "xoblite");
    }

    #[test]
    fn get_de_should_not_panic() {
        let _ = get_de(false, None);
    }

    #[test]
    fn get_de_should_exclude_wm_if_same() {
        let result = get_de(false, Some("Explorer"));
        assert!(result.is_none() || !result.as_ref().unwrap().eq_ignore_ascii_case("Explorer"));
    }
}
