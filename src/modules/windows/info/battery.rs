use crate::modules::utils::get_bar;

use std::str::FromStr;

use std::mem::MaybeUninit;
use winapi::um::winbase::GetSystemPowerStatus;
use winapi::um::winbase::SYSTEM_POWER_STATUS;

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum BatteryDisplayMode {
    Off,
    Bar,
    InfoBar,
    BarInfo,
}

impl FromStr for BatteryDisplayMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        match s.to_lowercase().as_str() {
            "off" => Ok(BatteryDisplayMode::Off),
            "bar" => Ok(BatteryDisplayMode::Bar),
            "infobar" => Ok(BatteryDisplayMode::InfoBar),
            "barinfo" => Ok(BatteryDisplayMode::BarInfo),
            _ => Ok(BatteryDisplayMode::BarInfo),
        }
    }
}

pub fn get_battery(display_mode: BatteryDisplayMode) -> Vec<String> {
    let mut status = MaybeUninit::<SYSTEM_POWER_STATUS>::uninit();

    let success = unsafe { GetSystemPowerStatus(status.as_mut_ptr()) };
    if success == 0 {
        return Vec::new();
    }

    let status = unsafe { status.assume_init() };

    let capacity = status.BatteryLifePercent;
    if capacity == 255 {
        return Vec::new(); // Unknown
    }

    let status_str = match status.ACLineStatus {
        0 => "Discharging",
        1 => "Charging",
        _ => "Unknown",
    };

    let bar = get_bar(capacity);

    let value = match display_mode {
        BatteryDisplayMode::Off => format!("{}% [{}]", capacity, status_str),
        BatteryDisplayMode::Bar => bar,
        BatteryDisplayMode::InfoBar => format!("{}% [{}] {}", capacity, status_str, bar),
        BatteryDisplayMode::BarInfo => format!("{} {}% [{}]", bar, capacity, status_str),
    };

    vec![format!("(Battery): {}", value)]
}
