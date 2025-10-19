use std::ptr::null_mut;
use winapi::shared::winerror::ERROR_SUCCESS;
use winapi::um::winreg::{HKEY_LOCAL_MACHINE, RRF_RT_REG_SZ, RegGetValueW};

pub fn get_model() -> Option<String> {
    // Read from BIOS registry branch (fast, no WMI):
    // HKLM\HARDWARE\DESCRIPTION\System\BIOS\SystemManufacturer
    // HKLM\HARDWARE\DESCRIPTION\System\BIOS\SystemProductName
    let manu = read_reg_sz("HARDWARE\\DESCRIPTION\\System\\BIOS", "SystemManufacturer")
        .unwrap_or_default();
    let prod =
        read_reg_sz("HARDWARE\\DESCRIPTION\\System\\BIOS", "SystemProductName").unwrap_or_default();
    // Prefer product alone when present; most vendors include brand already
    let raw = if !prod.trim().is_empty() {
        prod
    } else {
        format!("{} {}", manu, prod)
    };
    let cleaned = cleanup_model_string(&raw);
    if cleaned.is_empty() || cleaned == "Unknown" {
        None
    } else {
        Some(cleaned)
    }
}

fn cleanup_model_string(model: &str) -> String {
    let mut s = model.trim().to_string();

    let garbage = [
        "To be filled by O.E.M.",
        "To Be Filled",
        "OEM",
        "Not Applicable",
        "System Product Name",
        "System Version",
        "Undefined",
        "Default string",
        "Not Specified",
        "Type1ProductConfigId",
        "INVALID",
        "All Series",
    ];

    for g in garbage {
        s = s.replace(g, "").trim().to_string();
    }

    // Normalize noisy manufacturer prefixes if they remain
    let manu_map = [
        ("ASUSTeK COMPUTER INC.", "ASUS"),
        ("ASUSTeK COMPUTER INC", "ASUS"),
        ("ASUSTeK", "ASUS"),
        ("Hewlett-Packard", "HP"),
        ("HP Inc.", "HP"),
        ("Dell Inc.", "Dell"),
        ("LENOVO", "Lenovo"),
    ];
    for (from, to) in manu_map {
        if s.starts_with(from) {
            s = s.replacen(from, to, 1).trim().to_string();
        }
    }

    if s.starts_with("Standard PC") {
        return format!("KVM/QEMU ({})", model.trim());
    }

    if s.starts_with("OpenBSD") {
        return format!("vmm ({})", model.trim());
    }

    if s.is_empty() {
        return "Unknown".into();
    }

    s
}

fn read_reg_sz(subkey: &str, value: &str) -> Option<String> {
    let key = to_wide(subkey);
    let val = to_wide(value);
    let mut size: u32 = 0;
    let status = unsafe {
        RegGetValueW(
            HKEY_LOCAL_MACHINE,
            key.as_ptr(),
            val.as_ptr(),
            RRF_RT_REG_SZ,
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
            RRF_RT_REG_SZ,
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

fn to_wide(s: &str) -> Vec<u16> {
    use std::os::windows::ffi::OsStrExt;
    std::ffi::OsStr::new(s)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}
