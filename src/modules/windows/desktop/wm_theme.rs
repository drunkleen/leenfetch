use std::{ffi::OsStr, io::Error, os::windows::ffi::OsStrExt, process::Command, ptr};
use windows_sys::Win32::System::Registry::{
    RegOpenKeyExW, RegQueryValueExW, HKEY_CURRENT_USER, KEY_READ, REG_DWORD,
};

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
    hkey: windows_sys::Win32::Foundation::HKEY,
    path: &str,
    value: &str,
) -> Result<u32, Error> {
    use windows_sys::Win32::Foundation::LPBYTE;

    unsafe {
        let mut hkey_out = ptr::null_mut();
        let subkey_wide: Vec<u16> = OsStr::new(path).encode_wide().chain(Some(0)).collect();
        let valname_wide: Vec<u16> = OsStr::new(value).encode_wide().chain(Some(0)).collect();

        if RegOpenKeyExW(hkey, subkey_wide.as_ptr(), 0, KEY_READ, &mut hkey_out) != 0 {
            return Err(Error::last_os_error());
        }

        let mut data: u32 = 0;
        let mut data_size = std::mem::size_of::<u32>() as u32;
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
