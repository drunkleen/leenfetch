use crate::modules::{
    enums::{DiskDisplay, DiskSubtitle},
    utils::get_bar,
};
use std::process::Command;

pub fn get_disks(
    subtitle_mode: DiskSubtitle,
    display_mode: DiskDisplay,
    _paths: Option<Vec<&str>>,
) -> Option<Vec<(String, String)>> {
    let output = Command::new("wmic")
        .args(["logicaldisk", "get", "DeviceID,Size,FreeSpace"])
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    Some(parse_disk_output(&stdout, subtitle_mode, display_mode))
}

fn parse_disk_output(
    stdout: &str,
    subtitle_mode: DiskSubtitle,
    display_mode: DiskDisplay,
) -> Vec<(String, String)> {
    let lines = stdout.lines().skip(1); // skip header
    let mut results = Vec::new();

    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 3 {
            continue;
        }

        let mount = parts[0]; // "C:"

        let free = match parts[1].parse::<u64>() {
            Ok(v) => v,
            Err(_) => continue,
        };
        let total = match parts[2].parse::<u64>() {
            Ok(v) => v,
            Err(_) => continue,
        };

        if total == 0 {
            continue;
        }

        let used = total - free;
        let perc = ((used as f64 / total as f64) * 100.0).round() as u8;

        let used_str = format!("{:.1}G", used as f64 / 1024.0 / 1024.0 / 1024.0);
        let total_str = format!("{:.1}G", total as f64 / 1024.0 / 1024.0 / 1024.0);
        let usage_display = format!("{} / {}", used_str, total_str);

        let bar = get_bar(perc);
        let final_str = match display_mode {
            DiskDisplay::Info => usage_display.clone(),
            DiskDisplay::Percentage => format!("{}% {}", perc, bar),
            DiskDisplay::InfoBar => format!("{} {}", usage_display, bar),
            DiskDisplay::BarInfo => format!("{} {}", bar, usage_display),
            DiskDisplay::Bar => bar,
        };

        let subtitle = match subtitle_mode {
            DiskSubtitle::Name => mount.to_string(),
            DiskSubtitle::Dir => mount.trim_end_matches(':').to_string(),
            DiskSubtitle::Mount => mount.to_string(),
            DiskSubtitle::None => String::new(),
        };

        let label = if subtitle.is_empty() {
            "Disk".to_string()
        } else {
            format!("Disk ({})", subtitle)
        };

        results.push((label, final_str));
    }

    results
}
