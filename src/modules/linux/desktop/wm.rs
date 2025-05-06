use std::env;
use std::fs;
use std::process::Command;

pub fn get_wm(os: &str, kernel_name: &str) -> Option<String> {
    // Determine `ps` flags based on platform
    let ps_flags = if kernel_name.contains("OpenBSD") {
        vec!["x", "-c"]
    } else {
        vec!["-e"]
    };

    // Wayland check
    if let Ok(xdg_runtime_dir) = env::var("XDG_RUNTIME_DIR") {
        let socket = env::var("WAYLAND_DISPLAY").unwrap_or_else(|_| "wayland-0".to_string());
        let sock_path = format!("{}/{}", xdg_runtime_dir, socket);
        if let Ok(meta) = fs::metadata(&sock_path) {
            if meta.permissions().readonly() == false {
                // Try `lsof` or `fuser` substitute: scan known WM process names
                return scan_for_process(&ps_flags, WAYLAND_WMS);
            }
        }
    }

    // X11 check (DISPLAY set, not Mac/FreeMiNT)
    if env::var("DISPLAY").is_ok() && os != "Mac OS X" && os != "macOS" && os != "FreeMiNT" {
        if let Some(wm) = scan_for_process(&ps_flags, X11_WMS) {
            return Some(wm);
        }

        // Try xprop if installed
        if is_installed("xprop") {
            if let Some(wm) = get_wm_from_xprop() {
                return Some(wm);
            }
        }
    }

    // No WM detected
    None
}

fn is_installed(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn scan_for_process(ps_flags: &[&str], targets: &[&str]) -> Option<String> {
    let output = Command::new("ps").args(ps_flags).output().ok()?.stdout;
    let ps_output = String::from_utf8_lossy(&output);

    for line in ps_output.lines() {
        for &target in targets {
            if line.contains(target) {
                return Some(normalize_wm(target));
            }
        }
    }

    None
}

fn get_wm_from_xprop() -> Option<String> {
    let root_out = Command::new("xprop")
        .args(["-root", "-notype", "_NET_SUPPORTING_WM_CHECK"])
        .output()
        .ok()?;
    let id = String::from_utf8_lossy(&root_out.stdout);
    let win_id = id.trim().rsplit(' ').next()?;

    let wm_out = Command::new("xprop")
        .args([
            "-id",
            win_id,
            "-notype",
            "-len",
            "100",
            "-f",
            "_NET_WM_NAME",
            "8t",
        ])
        .output()
        .ok()?;

    let name = String::from_utf8_lossy(&wm_out.stdout);
    let cleaned = name.split('=').last()?.trim().trim_matches('"');
    Some(cleaned.to_string())
}

fn normalize_wm(wm: &str) -> String {
    match wm {
        "WINDOWMAKER" => "wmaker",
        "GNOME Shell" | "gnome-shell" => "Mutter",
        "kwin_x11" | "kwin_wayland" => "KWin",
        other => other,
    }
    .to_string()
}

// Lists of common WM process names
const WAYLAND_WMS: &[&str] = &[
    "arcan",
    "asc",
    "clayland",
    "dwc",
    "fireplace",
    "gnome-shell",
    "greenfield",
    "grefsen",
    "hikari",
    "kwin",
    "lipstick",
    "maynard",
    "mazecompositor",
    "motorcar",
    "orbital",
    "orbment",
    "perceptia",
    "river",
    "rustland",
    "sway",
    "ulubis",
    "velox",
    "wavy",
    "way-cooler",
    "wayfire",
    "wayhouse",
    "westeros",
    "westford",
    "weston",
];

const X11_WMS: &[&str] = &[
    "sowm",
    "catwm",
    "fvwm",
    "dwm",
    "2bwm",
    "monsterwm",
    "tinywm",
    "x11fs",
    "xmonad",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_wm_may_exist() {
        let wm = get_wm("Linux", "Linux");
        assert!(wm.is_none() || !wm.as_ref().unwrap().is_empty());
    }

    #[test]
    fn test_normalize_aliases() {
        assert_eq!(normalize_wm("WINDOWMAKER"), "wmaker");
        assert_eq!(normalize_wm("GNOME Shell"), "Mutter");
        assert_eq!(normalize_wm("gnome-shell"), "Mutter"); // ✅ Add this
        assert_eq!(normalize_wm("kwin_wayland"), "KWin");
        assert_eq!(normalize_wm("sway"), "sway");
    }
}
