use crate::modules::utils::get_bar;
use std::fs;
use std::str::FromStr;

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

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "off" => Ok(BatteryDisplayMode::Off),
            "bar" => Ok(BatteryDisplayMode::Bar),
            "infobar" => Ok(BatteryDisplayMode::InfoBar),
            "barinfo" => Ok(BatteryDisplayMode::BarInfo),
            _ => Ok(BatteryDisplayMode::BarInfo), // default
        }
    }
}

pub fn get_battery(display_mode: BatteryDisplayMode) -> Vec<String> {
    let entries = match fs::read_dir("/sys/class/power_supply/") {
        Ok(e) => e,
        Err(_) => return Vec::new(),
    };

    let mut results = Vec::with_capacity(2); // most systems have max 2 batteries

    for entry in entries.flatten() {
        let name = entry.file_name();
        let name_str = name.to_string_lossy();

        // Filter battery-like names
        if !(name_str.starts_with("BAT")
            || name_str == "axp288_fuel_gauge"
            || name_str.starts_with("CMB"))
        {
            continue;
        }

        let path = entry.path();

        let capacity = match fs::read_to_string(path.join("capacity")) {
            Ok(v) => match v.trim().parse::<u8>() {
                Ok(num) => num,
                Err(_) => continue,
            },
            Err(_) => continue,
        };

        let status = fs::read_to_string(path.join("status"))
            .ok()
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|| "Unknown".to_string());

        let line = match display_mode {
            BatteryDisplayMode::Bar => get_bar(capacity),
            BatteryDisplayMode::InfoBar => {
                format!("{}% [{}] {}", capacity, status, get_bar(capacity))
            }
            BatteryDisplayMode::BarInfo => {
                format!("{} {}% [{}]", get_bar(capacity), capacity, status)
            }
            BatteryDisplayMode::Off => format!("{}% [{}]", capacity, status),
        };

        results.push(format!("({}): {}", name_str, line));
    }

    results
}
