use std::{fs, process::Command};

use crate::modules::enums::PackageShorthand;

pub fn get_packages(shorthand: PackageShorthand) -> Option<String> {
    let mut packages = 0u64;
    let mut managers = vec![];
    let mut manager_string = vec![];

    if is_installed("dpkg") {
        if let Some(count) = count_dpkg_packages() {
            packages += count;
            managers.push(format!("{} ({})", count, "dpkg"));
            manager_string.push("dpkg");
        }
    }

    if is_installed("pacman") {
        if let Some(count) = count_pacman_packages() {
            packages += count;
            managers.push(format!("{} ({})", count, "pacman"));
            manager_string.push("pacman");
        }
    }

    if is_installed("rpm") {
        if let Some(count) = count_via_shell("rpm -qa | wc -l") {
            packages += count;
            managers.push(format!("{} ({})", count, "rpm"));
            manager_string.push("rpm");
        }
    }

    if is_installed("flatpak") {
        if let Some(count) = count_via_shell("flatpak list --app --columns=application | wc -l") {
            packages += count;
            managers.push(format!("{} ({})", count, "flatpak"));
            manager_string.push("flatpak");
        }
    }

    if is_installed("snap") && is_snapd_running() {
        if let Some(count) = count_via_shell("snap list | tail -n +2 | wc -l") {
            packages += count;
            managers.push(format!("{} ({})", count, "snap"));
            manager_string.push("snap");
        }
    }

    if packages == 0 {
        return None;
    }

    match shorthand {
        PackageShorthand::Off => Some(format!("{} total", packages)),
        PackageShorthand::On => Some(managers.join(", ")),
        PackageShorthand::Tiny => Some(format!("{} ({})", packages, manager_string.join(", "))),
    }
}

fn is_installed(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn is_snapd_running() -> bool {
    Command::new("ps")
        .args(["-eo", "comm="])
        .output()
        .map(|o| {
            String::from_utf8_lossy(&o.stdout)
                .lines()
                .any(|line| line.trim().eq_ignore_ascii_case("snapd"))
        })
        .unwrap_or(false)
}

fn count_dpkg_packages() -> Option<u64> {
    let status = fs::read_to_string("/var/lib/dpkg/status").ok()?;
    let count = status
        .lines()
        .filter(|line| line.starts_with("Package: "))
        .count() as u64;
    Some(count)
}

fn count_pacman_packages() -> Option<u64> {
    let entries = fs::read_dir("/var/lib/pacman/local").ok()?;
    let count = entries
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false))
        .count() as u64;
    Some(count)
}

fn count_via_shell(command: &str) -> Option<u64> {
    let output = Command::new("sh").arg("-c").arg(command).output().ok()?;
    if !output.status.success() {
        return None;
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout
        .trim()
        .split_whitespace()
        .last()
        .and_then(|s| s.parse::<u64>().ok())
}
