use crate::modules::{enums::BatteryDisplayMode, utils::get_bar};
use std::fs;
use std::path::Path;

pub fn get_battery(display_mode: BatteryDisplayMode) -> Vec<String> {
    collect_batteries(Path::new("/sys/class/power_supply/"), display_mode)
}

fn collect_batteries(root: &Path, display_mode: BatteryDisplayMode) -> Vec<String> {
    let entries = match fs::read_dir(root) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::enums::BatteryDisplayMode;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn setup_battery(temp_root: &Path, name: &str, capacity: &str, status: &str) {
        let battery_path = temp_root.join(name);
        fs::create_dir_all(&battery_path).unwrap();
        fs::write(battery_path.join("capacity"), capacity).unwrap();
        fs::write(battery_path.join("status"), status).unwrap();
    }

    #[test]
    fn collects_batteries_with_info_bar() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!("leenfetch_battery_test_{unique}"));
        setup_battery(&root, "BAT0", "75", "Charging");

        let entries = collect_batteries(&root, BatteryDisplayMode::InfoBar);
        assert_eq!(entries.len(), 1);
        let info = &entries[0];
        assert!(info.contains("(BAT0):"), "missing battery label: {info}");
        assert!(info.contains("75%"), "missing percentage: {info}");
        assert!(info.contains("Charging"), "missing status: {info}");

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn ignores_non_battery_entries() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!("leenfetch_battery_nonbattery_{unique}"));
        setup_battery(&root, "AC0", "100", "Full");
        setup_battery(&root, "BAT1", "55", "Discharging");

        let entries = collect_batteries(&root, BatteryDisplayMode::Off);
        assert_eq!(entries.len(), 1);
        assert!(entries[0].contains("BAT1"), "unexpected entries: {:?}", entries);

        fs::remove_dir_all(root).unwrap();
    }
}
