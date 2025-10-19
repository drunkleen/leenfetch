use std::ptr;

use winapi::um::wingdi::DEVMODEW;
use winapi::um::winuser::{
    ENUM_CURRENT_SETTINGS, EnumDisplaySettingsW, GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN,
};

pub fn get_resolution(refresh_rate: bool) -> Option<String> {
    unsafe {
        let width = GetSystemMetrics(SM_CXSCREEN);
        let height = GetSystemMetrics(SM_CYSCREEN);

        if width == 0 || height == 0 {
            return None;
        }

        let mut result = format!("{}x{}", width, height);

        if refresh_rate {
            let mut devmode: DEVMODEW = std::mem::zeroed();
            devmode.dmSize = std::mem::size_of::<DEVMODEW>() as u16;

            let success = EnumDisplaySettingsW(ptr::null(), ENUM_CURRENT_SETTINGS, &mut devmode);
            if success != 0 {
                result = format!("{} @ {}Hz", result, devmode.dmDisplayFrequency);
            }
        }

        Some(result)
    }
}
