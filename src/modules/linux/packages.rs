use std::process::Command;

use crate::modules::enums::PackageShorthand;

pub fn get_packages(shorthand: PackageShorthand) -> Option<String> {
    let mut packages = 0u64;
    let mut managers = vec![];
    let mut manager_string = vec![];

    if is_installed("dpkg") {
        if let Some(count) = count_lines(Command::new("dpkg-query").args(["-f", ".\n", "-W"])) {
            packages += count;
            managers.push(format!("{} ({})", count, "dpkg"));
            manager_string.push("dpkg");
        }
    }

    if is_installed("pacman") {
        if let Some(count) = count_lines(Command::new("pacman").args(["-Qq", "--color", "never"])) {
            packages += count;
            managers.push(format!("{} ({})", count, "pacman"));
            manager_string.push("pacman");
        }
    }

    if is_installed("rpm") {
        if let Some(count) = count_lines(Command::new("rpm").arg("-qa")) {
            packages += count;
            managers.push(format!("{} ({})", count, "rpm"));
            manager_string.push("rpm");
        }
    }

    if is_installed("flatpak") {
        if let Some(count) = count_lines(Command::new("flatpak").arg("list")) {
            packages += count;
            managers.push(format!("{} ({})", count, "flatpak"));
            manager_string.push("flatpak");
        }
    }

    if is_installed("snap") && is_snapd_running() {
        if let Some(mut count) = count_lines(Command::new("snap").arg("list")) {
            count = count.saturating_sub(1); // header row
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

fn count_lines(cmd: &mut Command) -> Option<u64> {
    let output = cmd.output().ok()?;
    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Some(stdout.lines().count() as u64)
}

fn is_snapd_running() -> bool {
    Command::new("ps")
        .arg("-e")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).contains("snapd"))
        .unwrap_or(false)
}
