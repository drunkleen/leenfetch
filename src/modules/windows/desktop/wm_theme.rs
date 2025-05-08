use std::{ffi::OsStr, io::Error, os::windows::ffi::OsStrExt, process::Command, ptr};
use winapi::um::winnt::KEY_READ;
use winapi::um::winnt::REG_DWORD;
use winapi::um::winreg::{RegOpenKeyExW, RegQueryValueExW, HKEY_CURRENT_USER};

pub fn get_wm_theme(_wm: &str, _de: Option<&str>) -> Option<String> {
    let mode = detect_light_dark_mode();
    let accent = detect_accent_color().unwrap_or("Unknown".to_string());

    Some(format!("{} theme, accent: {}", mode, accent))
}

fn detect_light_dark_mode() -> &'static str {
    // Read the "AppsUseLightTheme" registry value
    match read_reg_dword(
        HKEY_CURRENT_USER,
        r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize",
        "AppsUseLightTheme",
    ) {
        Ok(1) => "Light",
        Ok(0) => "Dark",
        _ => "Unknown",
    }
}

fn detect_accent_color() -> Option<String> {
    let output = Command::new("powershell")
        .args([
            "-NoProfile",
            "-Command",
            "Get-ItemProperty -Path HKCU:\\Software\\Microsoft\\Windows\\DWM | Select-Object -ExpandProperty ColorizationColor",
        ])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let raw = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if raw.is_empty() {
        return None;
    }

    // Try to parse ARGB and convert to hex
    if let Ok(val) = u32::from_str_radix(&raw, 10) {
        Some(format!("#{:06X}", val & 0xFFFFFF))
    } else {
        Some(raw)
    }
}

fn read_reg_dword(
    hkey: winapi::shared::minwindef::HKEY,
    path: &str,
    value: &str,
) -> Result<u32, Error> {
    use winapi::shared::minwindef::{DWORD, LPBYTE};

    unsafe {
        let mut hkey_out = ptr::null_mut();
        let subkey_wide: Vec<u16> = OsStr::new(path).encode_wide().chain(Some(0)).collect();
        let valname_wide: Vec<u16> = OsStr::new(value).encode_wide().chain(Some(0)).collect();

        if RegOpenKeyExW(hkey, subkey_wide.as_ptr(), 0, KEY_READ, &mut hkey_out) != 0 {
            return Err(Error::last_os_error());
        }

        let mut data: DWORD = 0;
        let mut data_size = std::mem::size_of::<DWORD>() as DWORD;
        let mut val_type = 0;

        let result = RegQueryValueExW(
            hkey_out,
            valname_wide.as_ptr(),
            ptr::null_mut(),
            &mut val_type,
            &mut data as *mut _ as LPBYTE,
            &mut data_size,
        );

        if result != 0 || val_type != REG_DWORD {
            return Err(Error::last_os_error());
        }

        Ok(data)
    }
}
