use std::env;
use std::fs;

pub fn get_wm() -> Option<String> {
    // Check environment variables first (fastest)
    if let Ok(swaysock) = env::var("SWAYSOCK") {
        if !swaysock.is_empty() {
            return Some("sway".to_string());
        }
    }

    if let Ok(hyprland) = env::var("HYPRLAND_INSTANCE_SIGNATURE") {
        if !hyprland.is_empty() {
            return Some("Hyprland".to_string());
        }
    }

    // Wayland detection via XDG_RUNTIME_DIR
    if let Ok(runtime) = env::var("XDG_RUNTIME_DIR") {
        let socket = env::var("WAYLAND_DISPLAY").unwrap_or_else(|_| "wayland-0".to_string());
        let path = format!("{}/{}", runtime, socket);
        if fs::metadata(&path).is_ok() {
            if let Some(wm) = scan_proc(WAYLAND_WMS) {
                return Some(wm);
            }
        }
    }

    // X11 (DISPLAY is set)
    if env::var("DISPLAY").is_ok() {
        // Scan known X11 WM processes
        if let Some(wm) = scan_proc(X11_WMS) {
            return Some(wm);
        }
    }

    // Fallback: scan all known WM processes
    scan_proc(ALL_WMS)
}

// Uses /proc to scan processes for known WMs (faster than spawning ps)
fn scan_proc(wm_names: &[&str]) -> Option<String> {
    let proc_path = match fs::read_dir("/proc") {
        Ok(p) => p,
        Err(_) => return None,
    };

    for entry in proc_path.flatten() {
        let name = entry.file_name();
        let name_str = name.to_string_lossy();

        // Only look at numeric PIDs
        if !name_str.chars().all(|c| c.is_ascii_digit()) {
            continue;
        }

        // Read the comm file (process name)
        let comm_path = entry.path().join("comm");
        if let Ok(comm) = fs::read_to_string(&comm_path) {
            let process_name = comm.trim();
            for &wm in wm_names {
                if process_name.eq_ignore_ascii_case(wm) || process_name.contains(wm) {
                    return Some(normalize_wm(wm));
                }
            }
        }
    }

    None
}

fn normalize_wm(wm: &str) -> String {
    match wm {
        "gnome-shell" | "GNOME Shell" => "Mutter",
        "kwin_x11" | "kwin_wayland" | "kwin" => "KWin",
        "WINDOWMAKER" => "wmaker",
        other => other,
    }
    .to_string()
}

// Common known Wayland WMs and compositors
const WAYLAND_WMS: &[&str] = &[
    "sway",
    "wayfire",
    "weston",
    "gnome-shell",
    "kwin_wayland",
    "hikari",
    "river",
    "wlr-randr",
];

// Common X11 WMs
const X11_WMS: &[&str] = &[
    "dwm",
    "xmonad",
    "openbox",
    "fluxbox",
    "i3",
    "herbstluftwm",
    "awesome",
    "bspwm",
    "kwin_x11",
];

// Combo of all known WMs
const ALL_WMS: &[&str] = &[
    "sway",
    "wayfire",
    "weston",
    "gnome-shell",
    "kwin_wayland",
    "kwin_x11",
    "dwm",
    "xmonad",
    "openbox",
    "fluxbox",
    "i3",
    "herbstluftwm",
    "awesome",
    "bspwm",
    "hikari",
    "river",
    "WINDOWMAKER",
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::EnvLock;

    fn clear_env() -> EnvLock {
        let vars = ["XDG_RUNTIME_DIR", "WAYLAND_DISPLAY", "DISPLAY"];
        let env_lock = EnvLock::acquire(&vars);
        for var in vars {
            env_lock.remove_var(var);
        }
        env_lock
    }

    #[test]
    fn test_normalize_wm_variants() {
        assert_eq!(normalize_wm("gnome-shell"), "Mutter");
        assert_eq!(normalize_wm("GNOME Shell"), "Mutter");
        assert_eq!(normalize_wm("kwin_x11"), "KWin");
        assert_eq!(normalize_wm("kwin_wayland"), "KWin");
        assert_eq!(normalize_wm("kwin"), "KWin");
        assert_eq!(normalize_wm("WINDOWMAKER"), "wmaker");
        assert_eq!(normalize_wm("i3"), "i3"); // fallback to default
    }

    #[test]
    fn test_get_wm_fallback_to_none_without_env() {
        let env_lock = clear_env();

        // Note: we can't test full get_wm() without actually scanning the system,
        // but we ensure it doesn't crash in a null environment
        let _ = get_wm(); // just call it and ensure no panic
        drop(env_lock);
    }
}
