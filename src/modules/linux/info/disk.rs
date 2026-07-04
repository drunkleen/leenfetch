use std::fs;

use crate::modules::{
    enums::{DiskDisplayPart, DiskSubtitle, parse_disk_display_parts},
    utils::get_bar,
};

pub fn get_disks(
    subtitle_mode: DiskSubtitle,
    display_mode: &str,
    paths: Option<Vec<&str>>,
) -> Option<Vec<(String, String)>> {
    // Get mount points - read /proc/mounts directly instead of spawning df
    let mount_points = if let Some(ref user_paths) = paths {
        user_paths.iter().map(|s| s.to_string()).collect()
    } else {
        // Default: pick the most useful visible mount per filesystem
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
    let mut points: Vec<(String, String)> = Vec::new();

    // Read /proc/mounts and keep only user-visible storage mounts.
    if let Ok(content) = fs::read_to_string("/proc/mounts") {
        for line in content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let mount = parts[0]; // Device
                let mount_point = parts[1];

                if mount_point == "/" || is_visible_storage_mount(mount_point) {
                    push_preferred_mount(&mut points, mount, mount_point);
                }
            }
        }
    }

    // Limit to avoid too many entries
    points.into_iter().map(|(_, mount)| mount).take(5).collect()
}

fn get_disk_info_for_path(
    path: &str,
    subtitle_mode: DiskSubtitle,
    display_mode: &str,
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

    let percent = ((used as f64 / total as f64) * 100.0)
        .round()
        .clamp(0.0, 100.0) as u8;

    // Format sizes in human-readable form
    let total_h = format_size(total);
    let used_h = format_size(used);

    let usage_display = format!("{} / {}", used_h, total_h);
    let perc_val = percent.min(100);

    let final_str = render_disk_display(display_mode, &usage_display, percent, perc_val);

    // Get device name from /proc/mounts
    let subtitle = match subtitle_mode {
        DiskSubtitle::Name => get_device_name(path),
        DiskSubtitle::Dir => mount_dir_label(path),
        DiskSubtitle::None => "".to_string(),
        DiskSubtitle::Mount => path.to_string(),
    };

    Some((subtitle, final_str))
}

fn render_disk_display(spec: &str, usage_display: &str, percent: u8, perc_val: u8) -> String {
    let parts = parse_disk_display_parts(spec).unwrap_or_else(|_| vec![DiskDisplayPart::Info]);

    let mut rendered = Vec::new();
    for part in parts {
        match part {
            DiskDisplayPart::Info => rendered.push(usage_display.to_string()),
            DiskDisplayPart::Percentage => rendered.push(format!("{}%", percent)),
            DiskDisplayPart::Bar => rendered.push(get_bar(perc_val)),
        }
    }

    rendered.join(" ")
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

fn is_visible_storage_mount(mount_point: &str) -> bool {
    matches!(
        mount_point,
        "/home" | "/data" | "/mnt" | "/media" | "/run/media"
    ) || mount_point.starts_with("/home/")
        || mount_point.starts_with("/data/")
        || mount_point.starts_with("/mnt/")
        || mount_point.starts_with("/media/")
        || mount_point.starts_with("/run/media/")
}

fn push_preferred_mount(points: &mut Vec<(String, String)>, device: &str, mount_point: &str) {
    let candidate = mount_point.to_string();

    if let Some((_, current_mount)) = points
        .iter_mut()
        .find(|(existing_device, _)| existing_device == device)
    {
        if mount_priority(&candidate) > mount_priority(current_mount) {
            *current_mount = candidate;
        }
        return;
    }

    points.push((device.to_string(), candidate));
}

fn mount_priority(path: &str) -> usize {
    let trimmed = path.trim_matches('/');
    if trimmed.is_empty() {
        0
    } else {
        trimmed.split('/').count() * 10 + trimmed.len()
    }
}

fn mount_dir_label(path: &str) -> String {
    path.trim_end_matches('/')
        .rsplit('/')
        .next()
        .filter(|segment| !segment.is_empty())
        .unwrap_or("")
        .to_string()
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

#[cfg(test)]
mod tests {
    use super::{
        is_visible_storage_mount, mount_priority, push_preferred_mount, render_disk_display,
    };

    #[test]
    fn keeps_user_visible_storage_mounts() {
        assert!(is_visible_storage_mount("/home"));
        assert!(is_visible_storage_mount("/home/snape"));
        assert!(is_visible_storage_mount("/data"));
        assert!(is_visible_storage_mount("/mnt/usb"));
        assert!(is_visible_storage_mount("/media/disk"));
        assert!(is_visible_storage_mount("/run/media/snape/USB"));
    }

    #[test]
    fn rejects_system_mounts() {
        assert!(!is_visible_storage_mount("/"));
        assert!(!is_visible_storage_mount("/boot"));
        assert!(!is_visible_storage_mount("/var"));
        assert!(!is_visible_storage_mount("/usr"));
        assert!(!is_visible_storage_mount("/opt"));
        assert!(!is_visible_storage_mount("/proc"));
        assert!(!is_visible_storage_mount("/sys"));
        assert!(!is_visible_storage_mount("/snap"));
        assert!(!is_visible_storage_mount("/dev/sda1"));
    }

    #[test]
    fn dir_label_uses_last_path_segment() {
        assert_eq!(mount_dir_label("/home"), "home");
        assert_eq!(mount_dir_label("/run/media/snape/USB"), "USB");
        assert_eq!(mount_dir_label("/mnt/external-drive/"), "external-drive");
        assert_eq!(mount_dir_label("/"), "");
    }

    #[test]
    fn prefers_more_useful_mount_for_same_device() {
        let mut points = Vec::new();

        push_preferred_mount(&mut points, "/dev/nvme0n1p2", "/");
        push_preferred_mount(&mut points, "/dev/nvme0n1p2", "/home");
        push_preferred_mount(&mut points, "/dev/nvme0n1p2", "/home/snape");

        assert_eq!(points.len(), 1);
        assert_eq!(points[0].1, "/home/snape");
        assert!(mount_priority("/home/snape") > mount_priority("/home"));
        assert!(mount_priority("/home") > mount_priority("/"));
    }

    #[test]
    fn renders_disk_display_parts_in_order() {
        assert_eq!(render_disk_display("percentage", "1G / 2G", 50, 50), "50%");
        assert_eq!(
            render_disk_display("info, percentage", "1G / 2G", 50, 50),
            "1G / 2G 50%"
        );
        assert_eq!(
            render_disk_display("percentage, bar", "1G / 2G", 50, 50),
            "50% ████████████████"
        );
    }
}

// fn parse_disk_output(
//     stdout: &str,
//     subtitle_mode: DiskSubtitle,
//     display_mode: &str,
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

//         let final_str = render_disk_display(display_mode, &usage_display, perc, perc_val);

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
