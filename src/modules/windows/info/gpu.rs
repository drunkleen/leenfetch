use std::ptr::null_mut;

use winapi::shared::guiddef::GUID;
use winapi::um::setupapi::{
    SetupDiDestroyDeviceInfoList, SetupDiEnumDeviceInfo, SetupDiGetClassDevsW,
    SetupDiGetDeviceRegistryPropertyW, DIGCF_PRESENT, SPDRP_DEVICEDESC, SP_DEVINFO_DATA,
};

pub fn get_gpus() -> Vec<String> {
    // Enumerate display adapters via SetupAPI (fast, no WMI/PowerShell)
    unsafe {
        let class_guid = GUID_DEVCLASS_DISPLAY;
        let hdev = SetupDiGetClassDevsW(&class_guid as *const _, null_mut(), null_mut(), DIGCF_PRESENT);
        if hdev.is_null() {
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
                let name = widestr_to_string(&buf);
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

fn widestr_to_string(buf: &[u16]) -> String {
    let len = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
    String::from_utf16_lossy(&buf[..len]).trim().to_string()
}

// {4d36e968-e325-11ce-bfc1-08002be10318}
const GUID_DEVCLASS_DISPLAY: GUID = GUID {
    Data1: 0x4d36e968,
    Data2: 0xe325,
    Data3: 0x11ce,
    Data4: [0xbf, 0xc1, 0x08, 0x00, 0x2b, 0xe1, 0x03, 0x18],
};
