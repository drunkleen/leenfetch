use std::process::Command;

use crate::modules::utils::get_bar;
use std::str::FromStr;

#[allow(dead_code)]
pub enum DiskSubtitle {
    Name,
    Dir,
    None,
    Mount,
}

impl FromStr for DiskSubtitle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "name" => Ok(DiskSubtitle::Name),
            "dir" => Ok(DiskSubtitle::Dir),
            "none" => Ok(DiskSubtitle::None),
            "mount" => Ok(DiskSubtitle::Mount),
            _ => Err(()),
        }
    }
}

#[allow(dead_code)]
pub enum DiskDisplay {
    Info,       // e.g. "40G / 80G"
    Percentage, // "50% [====      ]"
    InfoBar,    // "40G / 80G [====      ]"
    BarInfo,    // "[====      ] 40G / 80G"
    Bar,
}

impl FromStr for DiskDisplay {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "info" => Ok(DiskDisplay::Info),
            "percentage" => Ok(DiskDisplay::Percentage),
            "infobar" => Ok(DiskDisplay::InfoBar),
            "barinfo" => Ok(DiskDisplay::BarInfo),
            "bar" => Ok(DiskDisplay::Bar),
            _ => Err(()),
        }
    }
}

pub fn get_disks(
    subtitle_mode: DiskSubtitle,
    display_mode: DiskDisplay,
    paths: Option<Vec<&str>>,
) -> Option<Vec<(String, String)>> {
    let args: Vec<&str> = {
        let mut flags = vec!["-P", "-h"];
        if let Some(paths) = &paths {
            flags.extend(paths.iter());
        } else {
            flags.push("/");
        }
        flags
    };

    let output = Command::new("df").args(args).output().ok()?;
    let stdout = String::from_utf8_lossy(&output.stdout);

    Some(parse_disk_output(&stdout, subtitle_mode, display_mode))
}

fn parse_disk_output(
    stdout: &str,
    subtitle_mode: DiskSubtitle,
    display_mode: DiskDisplay,
) -> Vec<(String, String)> {
    let mut lines = stdout.lines().skip(1);
    let mut results = Vec::new();

    while let Some(line) = lines.next() {
        let parts: Vec<&str> = line.split_whitespace().filter(|s| !s.is_empty()).collect();

        if parts.len() < 6 {
            continue;
        }

        let total = parts[1];
        let used = parts[2];
        let perc = parts[4].trim_end_matches('%');
        let mount = parts[5];

        let usage_display = format!("{} / {}", used, total);
        let perc_val = perc.parse::<u8>().unwrap_or(0);

        let final_str = match display_mode {
            DiskDisplay::Info => usage_display,
            DiskDisplay::Percentage => format!("{}% {}", perc, get_bar(perc_val)),
            DiskDisplay::InfoBar => format!("{} {}", usage_display, get_bar(perc_val)),
            DiskDisplay::BarInfo => format!("{} {}", get_bar(perc_val), usage_display),
            DiskDisplay::Bar => get_bar(perc_val),
        };

        let subtitle = match subtitle_mode {
            DiskSubtitle::Name => parts[0].to_string(),
            DiskSubtitle::Dir => mount.split('/').last().unwrap_or("").to_string(),
            DiskSubtitle::None => "".to_string(),
            DiskSubtitle::Mount => mount.to_string(),
        };

        let full_subtitle = if subtitle.is_empty() {
            "Disk".to_string()
        } else {
            format!("Disk ({})", subtitle)
        };

        results.push((full_subtitle, final_str));
    }

    results
}

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
