use std::{env, fs, path::Path};

use std::ptr::null_mut;
use winapi::shared::minwindef::DWORD;
use winapi::shared::winerror::ERROR_SUCCESS;
use winapi::um::winreg::{RegGetValueW, HKEY_CURRENT_USER, RRF_RT_REG_DWORD};

pub fn get_theme(_de: Option<&str>) -> Option<String> {
    let mut result = Vec::new();

    // Check Windows theme preference
    if let Some(is_light) = read_windows_apps_theme() {
        let win_theme = if is_light { "Light" } else { "Dark" };
        result.push(format!("{win_theme} [Windows]"));
    }

    // Qt theme detection
    if let Ok(appdata) = env::var("APPDATA") {
        let qt_paths = [
            Path::new(&appdata).join("qt5ct/qt5ct.conf"),
            Path::new(&appdata).join("qt6ct/qt6ct.conf"),
        ];

        for path in qt_paths {
            if let Ok(content) = fs::read_to_string(&path) {
                for line in content.lines() {
                    if let Some(val) = line.trim().strip_prefix("style=") {
                        result.push(format!("{} [Qt]", val.trim()));
                        break;
                    }
                }
            }
        }
    }

    if result.is_empty() {
        None
    } else {
        Some(result.join(", "))
    }
}

fn read_windows_apps_theme() -> Option<bool> {
    let key_path = to_wide("Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize");
    let value_name = to_wide("AppsUseLightTheme");

    let mut data: DWORD = 0;
    let mut data_size = std::mem::size_of::<DWORD>() as u32;

    let status = unsafe {
        RegGetValueW(
            HKEY_CURRENT_USER,
            key_path.as_ptr(),
            value_name.as_ptr(),
            RRF_RT_REG_DWORD,
            null_mut(),
            &mut data as *mut _ as *mut _,
            &mut data_size,
        )
    };

    if status == ERROR_SUCCESS as i32 {
        Some(data != 0)
    } else {
        None
    }
}

fn to_wide(s: &str) -> Vec<u16> {
    use std::os::windows::ffi::OsStrExt;
    std::ffi::OsStr::new(s)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}
