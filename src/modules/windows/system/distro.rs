use std::env;
use std::ptr::null_mut;
use winapi::shared::winerror::ERROR_SUCCESS;
use winapi::um::winreg::{
    RegGetValueW, HKEY_LOCAL_MACHINE, RRF_RT_REG_SZ, RRF_SUBKEY_WOW6464KEY,
};

use crate::modules::enums::DistroDisplay;

pub fn get_distro(display: DistroDisplay) -> String {
    let name = get_product_name();
    let version = get_version();
    let arch = env::var("PROCESSOR_ARCHITECTURE").unwrap_or_else(|_| "unknown".into());
    let model = infer_windows_model(&name, &version);

    match display {
        DistroDisplay::Name => name,
        DistroDisplay::NameVersion => format!("{name} {version}").trim().to_string(),
        DistroDisplay::NameArch => format!("{name} {arch}").to_string(),
        DistroDisplay::NameModel => format!("{name} {model}").trim().to_string(),
        DistroDisplay::NameModelVersion => format!("{name} {model} {version}").trim().to_string(),
        DistroDisplay::NameModelArch => format!("{name} {model} {arch}").trim().to_string(),
        DistroDisplay::NameModelVersionArch => format!("{name} {model} {version} {arch}")
            .trim()
            .to_string(),
    }
}

fn get_product_name() -> String {
    // HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\ProductName
    let mut name = read_reg_sz(
        "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion",
        "ProductName",
    )
    .unwrap_or_else(|| "Windows".to_string());

    // Some environments (especially 32-bit builds on 64-bit OS) can expose an outdated
    // ProductName (e.g., "Windows 10 Pro") even on Windows 11. Distinguish by build number:
    // Windows 11 has build >= 22000.
    if let Some(build_s) = read_reg_sz(
        "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion",
        "CurrentBuildNumber",
    ) {
        if let Ok(build) = build_s.trim().parse::<u32>() {
            if build >= 22000 && name.contains("Windows 10") {
                name = name.replace("Windows 10", "Windows 11");
            }
        }
    }

    name
}

fn get_version() -> String {
    // Prefer DisplayVersion (e.g., 22H2), fallback to ReleaseId or build number
    if let Some(v) = read_reg_sz(
        "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion",
        "DisplayVersion",
    ) {
        return v;
    }
    if let Some(v) = read_reg_sz(
        "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion",
        "ReleaseId",
    ) {
        return v;
    }
    if let Some(b) = read_reg_sz(
        "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion",
        "CurrentBuildNumber",
    ) {
        return format!("Build {}", b);
    }
    "Unknown".into()
}

fn infer_windows_model(name: &str, version: &str) -> String {
    let n = name.to_lowercase();
    let v = version.to_lowercase();

    if n.contains("ltsc") || n.contains("enterprise lts") {
        return "LTS".to_string();
    }

    if n.contains("insider") {
        return "Testing".to_string();
    }

    if v.contains("dev") || v.contains("preview") {
        return "Preview".to_string();
    }

    "Stable".to_string()
}

fn read_reg_sz(subkey: &str, value: &str) -> Option<String> {
    let key = to_wide(subkey);
    let val = to_wide(value);
    // Helper to perform a single RegGetValueW read with provided flags
    fn read_with_flags(
        key: &[u16],
        val: &[u16],
        flags: u32,
    ) -> Option<String> {
        // Query buffer size
        let mut size: u32 = 0;
        let status = unsafe {
            RegGetValueW(
                HKEY_LOCAL_MACHINE,
                key.as_ptr(),
                val.as_ptr(),
                flags,
                null_mut(),
                null_mut(),
                &mut size,
            )
        };
        if status != ERROR_SUCCESS as i32 || size == 0 {
            return None;
        }
        let mut buf: Vec<u16> = vec![0u16; (size as usize + 1) / 2];
        let mut size2 = size;
        let status = unsafe {
            RegGetValueW(
                HKEY_LOCAL_MACHINE,
                key.as_ptr(),
                val.as_ptr(),
                flags,
                null_mut(),
                buf.as_mut_ptr() as *mut _,
                &mut size2,
            )
        };
        if status != ERROR_SUCCESS as i32 {
            return None;
        }
        let end = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
        Some(String::from_utf16_lossy(&buf[..end]).trim().to_string())
    }

    // Try 64-bit registry view first (helps x86 builds on x64 OS)
    read_with_flags(&key, &val, RRF_RT_REG_SZ | RRF_SUBKEY_WOW6464KEY)
        // Fallback to default view
        .or_else(|| read_with_flags(&key, &val, RRF_RT_REG_SZ))
}

fn to_wide(s: &str) -> Vec<u16> {
    use std::os::windows::ffi::OsStrExt;
    std::ffi::OsStr::new(s)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}
