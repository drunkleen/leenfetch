use std::process::Command;

/// Desktop Environment enum
pub fn get_de(show_version: bool, wm: Option<&str>) -> Option<String> {
    let tasklist = Command::new("tasklist").output().ok()?;
    let list = String::from_utf8_lossy(&tasklist.stdout).to_lowercase();

    for &entry in KNOWN_DES {
        if list.contains(entry) {
            let mut de = normalize_de(entry);

            // Avoid WM = DE collision
            if let Some(wm_name) = wm {
                if de.eq_ignore_ascii_case(wm_name) {
                    return None;
                }
            }

            // Optional version tag
            if show_version {
                if let Some(ver) = get_shell_version(entry) {
                    de.push(' ');
                    de.push_str(&ver);
                }
            }

            return Some(de);
        }
    }

    None
}

fn normalize_de(proc: &str) -> String {
    match proc {
        "explorer.exe" => "Explorer".to_string(),
        "cairodesktop.exe" => "Cairo".to_string(),
        "bblean.exe" => "BBLean".to_string(),
        "e17.exe" => "Enlightenment".to_string(),
        "xoblite.exe" => "xoblite".to_string(),
        _ => proc.trim_end_matches(".exe").to_string(),
    }
}

fn get_shell_version(shell_exe: &str) -> Option<String> {
    let output = Command::new("wmic")
        .args([
            "datafile",
            "where",
            &format!("name='C:\\\\Windows\\\\System32\\\\{}'", shell_exe),
            "get",
            "Version",
            "/value",
        ])
        .output()
        .ok()?;

    let out = String::from_utf8_lossy(&output.stdout);
    for line in out.lines() {
        if let Some(val) = line.strip_prefix("Version=") {
            return Some(val.trim().to_string());
        }
    }
    None
}

/// Known alternative or default Windows desktop environments / shells
const KNOWN_DES: &[&str] = &[
    "explorer.exe",     // Default shell
    "cairodesktop.exe", // Cairo Shell
    "bblean.exe",       // BlackBox for Windows
    "xoblite.exe",      // xoblite Blackbox fork
    "e17.exe",          // Enlightenment on Windows
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_known_shells() {
        assert_eq!(normalize_de("explorer.exe"), "Explorer");
        assert_eq!(normalize_de("cairodesktop.exe"), "Cairo");
        assert_eq!(normalize_de("xoblite.exe"), "xoblite");
    }

    #[test]
    fn get_de_should_not_panic() {
        let _ = get_de(false, None);
    }

    #[test]
    fn get_de_should_exclude_wm_if_same() {
        let result = get_de(false, Some("Explorer"));
        assert!(result.is_none() || !result.as_ref().unwrap().eq_ignore_ascii_case("Explorer"));
    }
}
