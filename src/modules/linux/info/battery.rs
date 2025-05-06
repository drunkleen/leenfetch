use crate::modules::utils::get_bar;
use std::fs;
use std::path::Path;
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
            _ => Err(()),
        }
    }
}

pub fn get_battery(display_mode: BatteryDisplayMode) -> Option<Vec<String>> {
    let base_path = Path::new("/sys/class/power_supply/");
    let entries = fs::read_dir(base_path).ok()?;

    let mut batteries = Vec::new();

    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();

        // Only consider batteries
        if !(name.starts_with("BAT") || name == "axp288_fuel_gauge" || name.starts_with("CMB")) {
            continue;
        }

        let bat_path = entry.path();

        let capacity_str = fs::read_to_string(bat_path.join("capacity")).ok()?;
        let status_str = fs::read_to_string(bat_path.join("status")).unwrap_or_default();

        let capacity: u8 = capacity_str.trim().parse().ok()?;
        let status = status_str.trim();

        let mut line = format!("{}% [{}]", capacity, status);

        match display_mode {
            BatteryDisplayMode::Bar => {
                line = get_bar(capacity);
            }
            BatteryDisplayMode::InfoBar => {
                line = format!("{} {}", line, get_bar(capacity));
            }
            BatteryDisplayMode::BarInfo => {
                line = format!("{} {}", get_bar(capacity), line);
            }
            BatteryDisplayMode::Off => {}
        }

        batteries.push(format!("Battery ({}): {}", name, line));
    }

    if batteries.is_empty() {
        None
    } else {
        Some(batteries)
    }
}
