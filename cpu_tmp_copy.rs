use std::ptr::null_mut;
use winapi::shared::minwindef::DWORD;
use winapi::shared::winerror::ERROR_SUCCESS;
use winapi::um::sysinfoapi::{GetSystemInfo, SYSTEM_INFO};
use winapi::um::winreg::{RegGetValueW, HKEY_LOCAL_MACHINE, RRF_RT_REG_DWORD, RRF_RT_REG_SZ};

pub fn get_cpu(
    cpu_brand: bool,
    show_speed: bool,
    show_cores: bool,
    show_temp: bool,
    speed_shorthand: bool,
    temp_unit: Option<char>,
) -> Option<String> {
    let mut output = get_cpu_model(cpu_brand);

    if show_cores {
        let cores = get_core_count();
        output = format!("{} ({})", output, cores);
    }

    if show_speed {
        if let Some(mhz) = get_cpu_speed_mhz() {
            let formatted = if mhz < 1000 {
                format!("{}MHz", mhz)
            } else {
                let mut ghz = mhz as f32 / 1000.0;
                if speed_shorthand {
                    ghz = (ghz * 10.0).round() / 10.0;
                }
                format!("{:.1}GHz", ghz)
            };
            output = format!("{} @ {}", output, formatted);
        }
    }

    if show_temp {
        if let Some(mut celsius) = get_cpu_temperature() {
            if let Some('F') = temp_unit {
                celsius = celsius * 9.0 / 5.0 + 32.0;
            }
            output = format!("{} [{:.1}°{}]", output, celsius, temp_unit.unwrap_or('C'));
        }
    }

    Some(output)
}

fn get_cpu_model(show_brand: bool) -> String {
    // Read from registry for speed and reliability
    // HKLM\HARDWARE\DESCRIPTION\System\CentralProcessor\0\ProcessorNameString
    if let Some(name) = read_reg_sz(
        HKEY_LOCAL_MACHINE,
        "HARDWARE\\DESCRIPTION\\System\\CentralProcessor\\0",
        "ProcessorNameString",
    ) {
        return sanitize_cpu_model(&name, show_brand);
    }

    "Unknown CPU".to_string()
}

fn get_core_count() -> u32 {
    // Use Win32 API to count logical processors across all groups.
    // This is fast and avoids WMI/PowerShell.
    unsafe {
        let mut info: SYSTEM_INFO = std::mem::zeroed();
        GetSystemInfo(&mut info as *mut _);
        let n = info.dwNumberOfProcessors;
        if n == 0 { 1 } else { n }
    }
}

fn get_cpu_speed_mhz() -> Option<u32> {
    // Read from registry: HKLM\HARDWARE\DESCRIPTION\System\CentralProcessor\0\~MHz
    let key = "HARDWARE\\DESCRIPTION\\System\\CentralProcessor\\0";
    let name = "~MHz";
    let mut data: DWORD = 0;
    let mut data_size = std::mem::size_of::<DWORD>() as u32;
    let status = unsafe {
        RegGetValueW(
            HKEY_LOCAL_MACHINE,
            to_wide(key).as_ptr(),
            to_wide(name).as_ptr(),
            RRF_RT_REG_DWORD,
            null_mut(),
            &mut data as *mut _ as *mut _,
            &mut data_size,
        )
    };
    if status == ERROR_SUCCESS as i32 {
        Some(data as u32)
    } else {
        None
    }
}

fn get_cpu_temperature() -> Option<f32> {
    // Windows does not expose a fast, reliable, non-admin CPU temp API.
    // Avoid WMI/CIM for performance; skip temperature.
    None
}

fn sanitize_cpu_model(model: &str, show_brand: bool) -> String {
    let mut s = model.to_string();

    let remove_patterns = [
        "(TM)",
        "(tm)",
        "(R)",
        "(r)",
        "CPU",
        "Processor",
        "with Radeon",
        "Technologies, Inc",
        "Core2",
        "Chip Revision",
        "Compute Cores",
        "FPU",
    ];

    for pat in remove_patterns.iter() {
        s = s.replace(pat, "");
    }

    if !show_brand {
        let brands = ["AMD ", "Intel ", "Qualcomm ", "Apple "];
        for b in brands {
            s = s.replacen(b, "", 1);
        }
    }

    s = s
        .split_whitespace()
        .filter(|word| {
            if let Some(stripped) = word.strip_suffix("-Core") {
                return !stripped.chars().all(|c| c.is_ascii_digit());
            }
            true
        })
        .collect::<Vec<_>>()
        .join(" ");

    s.trim().to_string()
}

fn to_wide(s: &str) -> Vec<u16> {
    use std::os::windows::ffi::OsStrExt;
    std::ffi::OsStr::new(s)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

fn read_reg_sz(root: winapi::shared::minwindef::HKEY, subkey: &str, value: &str) -> Option<String> {
    let sub = to_wide(subkey);
    let val = to_wide(value);
    // First query size
    let mut size: u32 = 0;
    let status = unsafe {
        RegGetValueW(
            root,
            sub.as_ptr(),
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
    // Allocate buffer of u16
    let len_wchars = (size as usize + 1) / 2; // bytes to wchar count
    let mut buf: Vec<u16> = vec![0u16; len_wchars];
    let mut size2 = size;
    let status = unsafe {
        RegGetValueW(
            root,
            sub.as_ptr(),
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
    // Find terminating 0
    let end = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
    Some(String::from_utf16_lossy(&buf[..end]).trim().to_string())
}

