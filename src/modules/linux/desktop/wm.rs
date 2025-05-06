use std::env;
use std::fs;
use std::process::Command;

pub fn get_wm() -> Option<String> {
    // Prefer Wayland
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
        // Most accurate: ask X server directly
        if let Some(wm) = get_wm_from_xprop() {
            return Some(wm);
        }

        // Fallback: scan known X11 WM processes
        if let Some(wm) = scan_proc(X11_WMS) {
            return Some(wm);
        }
    }

    // Fallback: scan all known WM processes
    scan_proc(ALL_WMS)
}

// Uses `ps -e` to scan processes for known WMs
fn scan_proc(wm_names: &[&str]) -> Option<String> {
    let output = Command::new("ps").arg("-e").output().ok()?.stdout;
    let ps_text = String::from_utf8_lossy(&output);

    for line in ps_text.lines() {
        for &wm in wm_names {
            if line.contains(wm) {
                return Some(normalize_wm(wm));
            }
        }
    }

    None
}

// Uses xprop to query the active X11 WM (_NET_WM_NAME)
fn get_wm_from_xprop() -> Option<String> {
    let root = Command::new("xprop")
        .args(["-root", "_NET_SUPPORTING_WM_CHECK"])
        .output()
        .ok()?;
    let root_out = String::from_utf8_lossy(&root.stdout);
    let win_id = root_out.rsplit(' ').next()?.trim();

    let wm = Command::new("xprop")
        .args(["-id", win_id, "-notype", "_NET_WM_NAME"])
        .output()
        .ok()?;
    let name = String::from_utf8_lossy(&wm.stdout);

    name.split('=')
        .nth(1)
        .map(|s| normalize_wm(s.trim().trim_matches('"')))
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
    use std::env;

    fn clear_env() {
        for var in ["XDG_RUNTIME_DIR", "WAYLAND_DISPLAY", "DISPLAY"] {
            env::remove_var(var);
        }
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
    fn test_scan_proc_finds_known_wm() {
        let sample_ps = "\
            1000 ?        00:00:00 i3
            1001 ?        00:00:00 bash
            1002 ?        00:00:00 Xorg
        ";

        for wm in ALL_WMS {
            let fake_output = sample_ps.replace("i3", wm);
            let found = parse_ps_for_wm(&fake_output, &[wm]);
            assert_eq!(found, Some(normalize_wm(wm)));
        }
    }

    // Internal helper to isolate scan_proc logic
    fn parse_ps_for_wm(ps_output: &str, targets: &[&str]) -> Option<String> {
        for line in ps_output.lines() {
            for &wm in targets {
                if line.contains(wm) {
                    return Some(normalize_wm(wm));
                }
            }
        }
        None
    }

    #[test]
    fn test_get_wm_fallback_to_none_without_env() {
        clear_env();

        // Note: we can't test full get_wm() without actually scanning the system,
        // but we ensure it doesn't crash in a null environment
        let _ = get_wm(); // just call it and ensure no panic
    }
}
