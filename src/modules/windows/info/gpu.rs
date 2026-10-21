use std::ptr::null_mut;

use windows_sys::Win32::Devices::DeviceAndDriverInstallation::{
    DIGCF_PRESENT, SP_DEVINFO_DATA, SPDRP_DEVICEDESC, SetupDiDestroyDeviceInfoList,
    SetupDiEnumDeviceInfo, SetupDiGetClassDevsW, SetupDiGetDeviceRegistryPropertyW,
};
use windows_sys::core::GUID;

pub fn get_gpus(gpu_brand: &str) -> Vec<String> {
    // Enumerate display adapters via SetupAPI (fast, no WMI/PowerShell)
    unsafe {
        let class_guid = GUID_DEVCLASS_DISPLAY;
        let hdev = SetupDiGetClassDevsW(
            &class_guid as *const _,
            null_mut(),
            null_mut(),
            DIGCF_PRESENT,
        );
        if hdev == 0 {
            return vec!["Unknown GPU".to_string()];
        }

        let mut index = 0u32;
        let mut names: Vec<String> = Vec::new();
        loop {
            let mut info: SP_DEVINFO_DATA = std::mem::zeroed();
            info.cbSize = std::mem::size_of::<SP_DEVINFO_DATA>() as u32;
            if SetupDiEnumDeviceInfo(hdev, index, &mut info) == 0 {
                break;
            }

            let mut buf: [u16; 512] = [0; 512];
            let mut req: u32 = 0;
            let ok = SetupDiGetDeviceRegistryPropertyW(
                hdev,
                &mut info,
                SPDRP_DEVICEDESC,
                null_mut(),
                buf.as_mut_ptr() as *mut u8,
                (buf.len() * 2) as u32,
                &mut req,
            );
            if ok != 0 {
                let name = format_gpu_name(&widestr_to_string(&buf), gpu_brand);
                if !name.is_empty() && !names.contains(&name) {
                    names.push(name);
                }
            }

            index += 1;
        }

        SetupDiDestroyDeviceInfoList(hdev);

        if names.is_empty() {
            vec!["Unknown GPU".to_string()]
        } else {
            names
        }
    }
}

fn format_gpu_name(name: &str, gpu_brand: &str) -> String {
    let vendor_and_model = name
        .strip_prefix("AMD ")
        .map(|model| ("Advanced Micro Devices", "AMD", model))
        .or_else(|| {
            name.strip_prefix("Advanced Micro Devices, Inc. ")
                .or_else(|| name.strip_prefix("Advanced Micro Devices "))
                .map(|model| ("Advanced Micro Devices", "AMD", model))
        })
        .or_else(|| {
            name.strip_prefix("NVIDIA Corporation ")
                .or_else(|| name.strip_prefix("NVIDIA "))
                .map(|model| ("NVIDIA Corporation", "NVIDIA", model))
        });

    match vendor_and_model {
        Some((full, _, model)) if gpu_brand.eq_ignore_ascii_case("full") => {
            format!("{full} {model}")
        }
        Some((_, short, model)) if gpu_brand.eq_ignore_ascii_case("short") => {
            format!("{short} {model}")
        }
        Some((_, _, model)) if gpu_brand.eq_ignore_ascii_case("off") => model.to_string(),
        _ => name.to_string(),
    }
}

fn widestr_to_string(buf: &[u16]) -> String {
    let len = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
    String::from_utf16_lossy(&buf[..len]).trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::format_gpu_name;

    #[test]
    fn formats_amd_and_nvidia_vendor_labels() {
        assert_eq!(
            format_gpu_name("AMD Radeon RX 7600", "full"),
            "Advanced Micro Devices Radeon RX 7600"
        );
        assert_eq!(
            format_gpu_name("NVIDIA Corporation GeForce RTX 5080", "short"),
            "NVIDIA GeForce RTX 5080"
        );
        assert_eq!(
            format_gpu_name("NVIDIA GeForce RTX 5080", "off"),
            "GeForce RTX 5080"
        );
    }
}

// {4d36e968-e325-11ce-bfc1-08002be10318}
const GUID_DEVCLASS_DISPLAY: GUID = GUID {
    data1: 0x4d36e968,
    data2: 0xe325,
    data3: 0x11ce,
    data4: [0xbf, 0xc1, 0x08, 0x00, 0x2b, 0xe1, 0x03, 0x18],
};
