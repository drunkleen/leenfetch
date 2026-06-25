use std::fs;
use std::path::Path;

use crate::modules::enums::PackageShorthand;

pub fn get_packages(shorthand: PackageShorthand) -> Option<String> {
    let mut packages = 0u64;
    let mut managers = vec![];
    let mut manager_string = vec![];

    // dpkg (Debian/Ubuntu)
    if let Some(count) = count_dpkg_packages() {
        packages += count;
        managers.push(format!("{} ({})", count, "dpkg"));
        manager_string.push("dpkg");
    }

    // pacman (Arch)
    if let Some(count) = count_pacman_packages() {
        packages += count;
        managers.push(format!("{} ({})", count, "pacman"));
        manager_string.push("pacman");
    }

    // rpm (RHEL/Fedora) - check multiple possible locations
    if let Some(count) = count_rpm_packages() {
        packages += count;
        managers.push(format!("{} ({})", count, "rpm"));
        manager_string.push("rpm");
    }

    // flatpak
    if let Some(count) = count_flatpak_packages() {
        packages += count;
        managers.push(format!("{} ({})", count, "flatpak"));
        manager_string.push("flatpak");
    }

    // snap - check if snapd is running via socket
    if is_snapd_running() {
        if let Some(count) = count_snap_packages() {
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

fn is_snapd_running() -> bool {
    // Check for snapd socket instead of running ps
    Path::new("/run/snapd.socket").exists() || Path::new("/var/run/snapd.socket").exists()
}

fn pkg_root() -> String {
    std::env::var("LEENFETCH_PKG_ROOT").unwrap_or_default()
}

fn count_dpkg_packages() -> Option<u64> {
    let root = pkg_root();
    let status = fs::read_to_string(format!("{root}/var/lib/dpkg/status")).ok()?;
    let count = status
        .lines()
        .filter(|line| line.starts_with("Package: "))
        .count() as u64;
    Some(count)
}

fn count_pacman_packages() -> Option<u64> {
    let root = pkg_root();
    let entries = fs::read_dir(format!("{root}/var/lib/pacman/local")).ok()?;
    let count = entries
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false))
        .count() as u64;
    Some(count)
}

fn count_rpm_packages() -> Option<u64> {
    let root = pkg_root();
    // Try /var/lib/rpm first (RPM DB)
    if let Ok(db_path) = fs::read_dir(format!("{root}/var/lib/rpm")) {
        // Count packages in RPM database
        for entry in db_path.flatten() {
            let name = entry.file_name();
            if name.to_string_lossy().starts_with("Packages") {
                // This is the RPM database - count entries
                if let Ok(content) = fs::read_to_string(entry.path()) {
                    return Some(content.lines().filter(|l| !l.is_empty()).count() as u64);
                }
            }
        }
    }

    // Fallback: try to count from /var/cache/Packages for apt-based systems
    if let Ok(entries) = fs::read_dir(format!("{root}/var/cache/apt")) {
        let count = entries.filter_map(|e| e.ok()).count() as u64;
        if count > 0 {
            return Some(count);
        }
    }

    None
}

fn count_flatpak_packages() -> Option<u64> {
    let root = pkg_root();
    // Check flatpak installation directories
    let paths = [
        format!("{root}/var/lib/flatpak/app"),
        format!("{root}/home/.local/share/flatpak/app"),
    ];

    for path in &paths {
        if let Ok(entries) = fs::read_dir(&path) {
            let count = entries
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_dir())
                .count() as u64;
            if count > 0 {
                return Some(count);
            }
        }
    }

    // Try system-wide installations
    if let Ok(home) = std::env::var("HOME") {
        let user_path = format!("{root}{home}/.local/share/flatpak/app");
        if let Ok(entries) = fs::read_dir(&user_path) {
            let count = entries
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_dir())
                .count() as u64;
            if count > 0 {
                return Some(count);
            }
        }
    }

    None
}

fn count_snap_packages() -> Option<u64> {
    let root = pkg_root();
    // Read snap list from /var/lib/snapd/state.json or direct snap data
    let snap_data_path = format!("{root}/var/lib/snapd/state.json");

    if let Ok(content) = fs::read_to_string(&snap_data_path) {
        // Try to parse JSON and count installed snaps
        // Simplified: count "name" occurrences in the JSON
        let count = content.matches("\"name\":").count() as u64;
        if count > 0 {
            return Some(count.saturating_sub(1)); // Subtract potential false positive
        }
    }

    // Fallback: count snap directories
    if let Ok(entries) = fs::read_dir(format!("{root}/snap")) {
        let count = entries
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_dir() && e.file_name() != "snap")
            .count() as u64;
        if count > 0 {
            return Some(count);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::EnvLock;

    #[test]
    fn returns_none_when_no_managers_found() {
        let env_lock = EnvLock::acquire(&["LEENFETCH_PKG_ROOT"]);
        env_lock.set_var("LEENFETCH_PKG_ROOT", "/nonexistent");
        let result = get_packages(PackageShorthand::Off);
        assert!(result.is_none());
        drop(env_lock);
    }
}
