use std::fs;

use crate::modules::{
    enums::{DiskDisplay, DiskSubtitle},
    utils::get_bar,
};

pub fn get_disks(
    subtitle_mode: DiskSubtitle,
    display_mode: DiskDisplay,
    paths: Option<Vec<&str>>,
) -> Option<Vec<(String, String)>> {
    // Get mount points - read /proc/mounts directly instead of spawning df
    let mount_points = if let Some(ref user_paths) = paths {
        user_paths.iter().map(|s| s.to_string()).collect()
    } else {
        // Default: get root and common mount points
        get_default_mount_points()
    };

    let mut results = Vec::new();

    for mount_point in mount_points {
        if let Some(disk_info) = get_disk_info_for_path(&mount_point, subtitle_mode, &display_mode)
        {
            results.push(disk_info);
        }
    }

    if results.is_empty() {
        return None;
    }

    Some(results)
}

fn get_default_mount_points() -> Vec<String> {
    let mut points = vec!["/".to_string()];
    let mut seen_devices: std::collections::HashSet<String> = std::collections::HashSet::new();

    // Read /proc/mounts to find common mount points
    if let Ok(content) = fs::read_to_string("/proc/mounts") {
        let common_prefixes = ["/home", "/boot", "/var", "/usr", "/opt", "/data"];

        for line in content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let mount = parts[0]; // Device
                let mount_point = parts[1];

                // Skip if we've already seen this device (same filesystem)
                if seen_devices.contains(mount) {
                    continue;
                }

                // Skip pseudo filesystems and already added paths
                if !mount_point.starts_with("/dev")
                    && !mount_point.starts_with("/sys")
                    && !mount_point.starts_with("/proc")
                    && !mount_point.starts_with("/run")
                    && !mount_point.starts_with("/snap")
                {
                    // Only add if it's a significant mount point
                    for prefix in &common_prefixes {
                        if mount_point.starts_with(prefix) && mount_point != "/" {
                            seen_devices.insert(mount.to_string());
                            points.push(mount_point.to_string());
                            break;
                        }
                    }
                }
            }
        }
    }

    // Limit to avoid too many entries
    points.truncate(5);
    points
}

fn get_disk_info_for_path(
    path: &str,
    subtitle_mode: DiskSubtitle,
    display_mode: &DiskDisplay,
) -> Option<(String, String)> {
    let path_cstr = std::ffi::CString::new(path).ok()?;

    // Use statvfs to get disk usage
    let mut statfs: libc::statvfs = unsafe { std::mem::zeroed() };

    if unsafe { libc::statvfs(path_cstr.as_ptr(), &mut statfs) } != 0 {
        return None;
    }

    // Calculate sizes in bytes
    let total = statfs.f_blocks as u64 * statfs.f_frsize as u64;
    let available = statfs.f_bavail as u64 * statfs.f_frsize as u64;
    let used = total.saturating_sub(available);

    if total == 0 {
        return None;
    }

    let percent = ((used as f64 / total as f64) * 100.0) as u8;

    // Format sizes in human-readable form
    let total_h = format_size(total);
    let used_h = format_size(used);

    let usage_display = format!("{} / {}", used_h, total_h);
    let perc_val = percent.min(100);

    let final_str = match display_mode {
        DiskDisplay::Info => usage_display,
        DiskDisplay::Percentage => format!("{}% {}", percent, get_bar(perc_val)),
        DiskDisplay::InfoBar => format!("{} {}", usage_display, get_bar(perc_val)),
        DiskDisplay::BarInfo => format!("{} {}", get_bar(perc_val), usage_display),
        DiskDisplay::Bar => get_bar(perc_val),
    };

    // Get device name from /proc/mounts
    let subtitle = match subtitle_mode {
        DiskSubtitle::Name => get_device_name(path),
        DiskSubtitle::Dir => path
            .trim_start_matches('/')
            .split('/')
            .next()
            .unwrap_or("")
            .to_string(),
        DiskSubtitle::None => "".to_string(),
        DiskSubtitle::Mount => path.to_string(),
    };

    let full_subtitle = if subtitle.is_empty() {
        "Disk".to_string()
    } else {
        format!("Disk ({})", subtitle)
    };

    Some((full_subtitle, final_str))
}

fn get_device_name(path: &str) -> String {
    if let Ok(content) = fs::read_to_string("/proc/mounts") {
        for line in content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 && parts[1] == path {
                return parts[0].to_string();
            }
        }
    }
    "".to_string()
}

fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    if bytes >= TB {
        format!("{:.1}T", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.1}G", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1}M", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1}K", bytes as f64 / KB as f64)
    } else {
        format!("{}B", bytes)
    }
}

// fn parse_disk_output(
//     stdout: &str,
//     subtitle_mode: DiskSubtitle,
//     display_mode: DiskDisplay,
// ) -> Vec<(String, String)> {
//     let mut lines = stdout.lines().skip(1);
//     let mut results = Vec::new();

//     while let Some(line) = lines.next() {
//         let parts: Vec<&str> = line.split_whitespace().filter(|s| !s.is_empty()).collect();

//         if parts.len() < 6 {
//             continue;
//         }

//         let total = parts[1];
//         let used = parts[2];
//         let perc = parts[4].trim_end_matches('%');
//         let mount = parts[5];

//         let usage_display = format!("{} / {}", used, total);
//         let perc_val = perc.parse::<u8>().unwrap_or(0);

//         let final_str = match display_mode {
//             DiskDisplay::Info => usage_display,
//             DiskDisplay::Percentage => format!("{}% {}", perc, get_bar(perc_val)),
//             DiskDisplay::InfoBar => format!("{} {}", usage_display, get_bar(perc_val)),
//             DiskDisplay::BarInfo => format!("{} {}", get_bar(perc_val), usage_display),
//             DiskDisplay::Bar => get_bar(perc_val),
//         };

//         let subtitle = match subtitle_mode {
//             DiskSubtitle::Name => parts[0].to_string(),
//             DiskSubtitle::Dir => mount.split('/').last().unwrap_or("").to_string(),
//             DiskSubtitle::None => "".to_string(),
//             DiskSubtitle::Mount => mount.to_string(),
//         };

//         let full_subtitle = if subtitle.is_empty() {
//             "Disk".to_string()
//         } else {
//             format!("Disk ({})", subtitle)
//         };

//         results.push((full_subtitle, final_str));
//     }

//     results
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_df_output() -> &'static str {
        r#"Filesystem     Size  Used Avail Use% Mounted on
/dev/sda1       100G   40G   60G  40% /
tmpfs           2.0G     0  2.0G   0% /dev/shm
/dev/sdb1       200G  150G   50G  75% /mnt/data
"#
    }

    #[test]
    fn test_parse_info_display_mount_subtitle() {
        let result = parse_disk_output(sample_df_output(), DiskSubtitle::Mount, DiskDisplay::Info);

        assert_eq!(result.len(), 3);
        assert_eq!(result[0].0, "Disk (/)");
        assert_eq!(result[0].1, "40G / 100G");
    }

    #[test]
    fn test_parse_percentage_display_dir_subtitle() {
        let result = parse_disk_output(
            sample_df_output(),
            DiskSubtitle::Dir,
            DiskDisplay::Percentage,
        );

        assert_eq!(result[2].0, "Disk (data)");
        assert!(result[2].1.starts_with("75%"));
    }

    #[test]
    fn test_parse_bar_only() {
        let result = parse_disk_output(sample_df_output(), DiskSubtitle::None, DiskDisplay::Bar);

        assert_eq!(result[0].0, "Disk");
        assert!(result[0].1.starts_with("[") && result[0].1.ends_with("]"));
        assert!(result[0].1.len() > 2); // should contain some kind of bar
    }

    #[test]
    fn test_parse_barinfo_with_name() {
        let result =
            parse_disk_output(sample_df_output(), DiskSubtitle::Name, DiskDisplay::BarInfo);

        assert_eq!(result[2].0, "Disk (/dev/sdb1)");
        assert!(result[2].1.contains("150G / 200G"));
    }
}
