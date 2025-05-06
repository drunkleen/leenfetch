use std::env;

use std::process::Command;

pub fn get_de(_os: &str, _distro: &str, wm: Option<&str>, show_version: bool) -> Option<String> {
    let mut de: Option<String> = None;

    // Primary: XDG environment variables
    if let Ok(session) = env::var("DESKTOP_SESSION") {
        if session.contains("regolith") {
            de = Some("Regolith".to_string());
        } else {
            de = Some(session);
        }
    }

    if de.is_none() {
        if let Ok(current) = env::var("XDG_CURRENT_DESKTOP") {
            let cleaned = current.replace("X-", "").replace("Budgie:GNOME", "Budgie");
            de = Some(cleaned);
        }
    }

    if de.is_none() {
        if env::var("GNOME_DESKTOP_SESSION_ID").is_ok() {
            de = Some("GNOME".to_string());
        } else if env::var("MATE_DESKTOP_SESSION_ID").is_ok() {
            de = Some("MATE".to_string());
        } else if env::var("TDE_FULL_SESSION").is_ok() {
            de = Some("Trinity".to_string());
        }
    }

    // Prevent WM == DE
    if let Some(de_val) = &de {
        if let Some(wm_val) = wm {
            if de_val == wm_val {
                de = None;
            }
        }
    }

    // xprop fallback (if installed)
    if de.is_none() && env::var("DISPLAY").is_ok() && is_installed("xprop") {
        if let Some(xprop_out) = run_command("xprop", &["-root"]) {
            if xprop_out.contains("KDE_SESSION_VERSION") {
                de = Some("KDE".to_string());
            } else if xprop_out.contains("_MUFFIN") {
                de = Some("Cinnamon".to_string());
            } else if xprop_out.contains("xfce") {
                de = Some("Xfce".to_string());
            }
        }
    }

    // Format
    if let Some(de_name) = &mut de {
        if de_name.contains("xfce4") {
            *de_name = "Xfce4".to_string();
        } else if de_name.contains("xfce5") {
            *de_name = "Xfce5".to_string();
        } else if de_name.contains("xfce") {
            *de_name = "Xfce".to_string();
        } else if de_name.contains("mate") {
            *de_name = "MATE".to_string();
        } else if de_name.to_lowercase().contains("gnome") {
            *de_name = "GNOME".to_string();
        } else if de_name.contains("MUFFIN") {
            *de_name = "Cinnamon".to_string();
        }

        if env::var("KDE_SESSION_VERSION").map_or(false, |v| v.parse::<u32>().unwrap_or(0) >= 4) {
            *de_name = de_name.replace("KDE", "Plasma");
        }

        if show_version {
            if let Some(ver) = get_de_version(de_name) {
                *de_name = format!("{} {}", de_name, ver);
            }
        }

        if env::var("WAYLAND_DISPLAY").is_ok() {
            *de_name += " (Wayland)";
        }
    }

    de
}

fn is_installed(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn run_command(cmd: &str, args: &[&str]) -> Option<String> {
    Command::new(cmd)
        .args(args)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
}

fn get_de_version(de: &str) -> Option<String> {
    match de {
        "Plasma" => parse_version("plasmashell", &["--version"]),
        "MATE" => parse_version("mate-session", &["--version"]),
        "Xfce" | "Xfce4" => parse_version("xfce4-session", &["--version"]),
        "GNOME" => parse_version("gnome-shell", &["--version"]),
        "Cinnamon" => parse_version("cinnamon", &["--version"]),
        "Budgie" => parse_version("budgie-desktop", &["--version"]),
        "LXQt" => parse_version("lxqt-session", &["--version"]),
        "Lumina" => parse_version("lumina-desktop", &["--version"]),
        "Trinity" => parse_version("tde-config", &["--version"]),
        "Unity" => parse_version("unity", &["--version"]),
        _ => None,
    }
}

fn parse_version(command: &str, args: &[&str]) -> Option<String> {
    run_command(command, args).map(|out| {
        out.lines()
            .last()
            .unwrap_or("")
            .split_whitespace()
            .find(|s| s.chars().next().map_or(false, |c| c.is_ascii_digit()))
            .unwrap_or("")
            .to_string()
    })
}

#[test]
fn test_de_from_desktop_session() {
    with_clean_env(|| {
        std::env::set_var("DESKTOP_SESSION", "xfce");
        let result = get_de("Linux", "", None, false);
        assert_eq!(result.unwrap(), "Xfce");
    });
}

#[test]
fn test_de_from_gnome_fallback() {
    with_clean_env(|| {
        std::env::set_var("GNOME_DESKTOP_SESSION_ID", "true");
        let result = get_de("Linux", "", None, false);

        assert!(result.is_some(), "Expected Some(DE), got None");
        assert_eq!(result.unwrap(), "GNOME");
    });
}

#[allow(dead_code)]
fn with_clean_env<F: FnOnce()>(test: F) {
    for var in [
        "DESKTOP_SESSION",
        "XDG_CURRENT_DESKTOP",
        "GNOME_DESKTOP_SESSION_ID",
        "MATE_DESKTOP_SESSION_ID",
        "TDE_FULL_SESSION",
        "KDE_SESSION_VERSION",
        "WAYLAND_DISPLAY",
        "DISPLAY",
    ] {
        std::env::remove_var(var);
    }

    test();

    for var in [
        "DESKTOP_SESSION",
        "XDG_CURRENT_DESKTOP",
        "GNOME_DESKTOP_SESSION_ID",
        "MATE_DESKTOP_SESSION_ID",
        "TDE_FULL_SESSION",
        "KDE_SESSION_VERSION",
        "WAYLAND_DISPLAY",
        "DISPLAY",
    ] {
        std::env::remove_var(var);
    }
}
