use std::env;
use std::process::Command;

pub fn get_de(show_version: bool, wm: Option<&str>) -> Option<String> {
    let mut de = detect_de_env().or_else(detect_de_fallback)?;

    // Avoid false-positive where WM == DE
    if let Some(wm_name) = wm {
        if de.eq_ignore_ascii_case(wm_name) {
            return None;
        }
    }

    normalize_de_name(&mut de);

    // Detect version if requested
    if show_version {
        if let Some(ver) = get_de_version(&de) {
            de = format!("{} {}", de, ver);
        }
    }

    // Tag if under Wayland
    if env::var_os("WAYLAND_DISPLAY").is_some() {
        de.push_str(" (Wayland)");
    }

    Some(de)
}

/// Primary DE env detection via standard XDG env vars
fn detect_de_env() -> Option<String> {
    if let Some(val) = env::var_os("DESKTOP_SESSION") {
        let val_str = val.to_string_lossy();
        return Some(if val_str.contains("regolith") {
            "Regolith".into()
        } else {
            val_str.into()
        });
    }

    if let Some(val) = env::var_os("XDG_CURRENT_DESKTOP") {
        let val_str = val
            .to_string_lossy()
            .replace("X-", "")
            .replace("Budgie:GNOME", "Budgie");
        return Some(val_str);
    }

    None
}

/// Fallback: legacy DE-specific env vars
fn detect_de_fallback() -> Option<String> {
    if env::var_os("GNOME_DESKTOP_SESSION_ID").is_some() {
        return Some("GNOME".into());
    }
    if env::var_os("MATE_DESKTOP_SESSION_ID").is_some() {
        return Some("MATE".into());
    }
    if env::var_os("TDE_FULL_SESSION").is_some() {
        return Some("Trinity".into());
    }

    // X11 fallback using xprop if available
    if env::var_os("DISPLAY").is_some() && is_installed("xprop") {
        let output = run_command("xprop", &["-root"])?;
        if output.contains("KDE_SESSION_VERSION") {
            return Some("KDE".into());
        }
        if output.contains("_MUFFIN") {
            return Some("Cinnamon".into());
        }
        if output.contains("xfce") {
            return Some("Xfce".into());
        }
    }

    None
}

/// Normalize strings like "xfce4" -> "Xfce"
fn normalize_de_name(de: &mut String) {
    let lower = de.to_lowercase();
    *de = match lower.as_str() {
        s if s.contains("xfce4") => "Xfce4",
        s if s.contains("xfce5") => "Xfce5",
        s if s.contains("xfce") => "Xfce",
        s if s.contains("mate") => "MATE",
        s if s.contains("gnome") => "GNOME",
        s if s.contains("muffin") => "Cinnamon",
        s if s.contains("budgie") => "Budgie",
        s if s.contains("lxqt") => "LXQt",
        s if s.contains("plasma") || s.contains("kde") => "Plasma",
        s if s.contains("unity") => "Unity",
        _ => de.as_str(),
    }
    .to_string();
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

fn parse_version(cmd: &str, args: &[&str]) -> Option<String> {
    run_command(cmd, args).and_then(|out| {
        out.lines().rev().find_map(|line| {
            line.split_whitespace()
                .find(|s| {
                    s.chars()
                        .next()
                        .map(|c| c.is_ascii_digit())
                        .unwrap_or(false)
                })
                .map(|s| s.to_string())
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::EnvLock;

    fn clear_env() -> EnvLock {
        let vars = [
            "DESKTOP_SESSION",
            "XDG_CURRENT_DESKTOP",
            "GNOME_DESKTOP_SESSION_ID",
            "MATE_DESKTOP_SESSION_ID",
            "TDE_FULL_SESSION",
            "WAYLAND_DISPLAY",
            "DISPLAY",
        ];
        let env_lock = EnvLock::acquire(&vars);
        for var in vars {
            env_lock.remove_var(var);
        }
        env_lock
    }

    // #[test]
    // fn detects_de_from_desktop_session() {
    //     clear_env();
    //     env::set_var("DESKTOP_SESSION", "xfce4");
    //     let result = get_de(false, None);
    //     assert_eq!(result, Some("Xfce4".to_string()));
    // }

    // #[test]
    // fn detects_regolith_special_case() {
    //     clear_env();
    //     env::set_var("DESKTOP_SESSION", "regolith-session");
    //     let result = get_de(false, None);
    //     assert_eq!(result, Some("Regolith".to_string()));
    // }

    // #[test]
    // fn detects_de_from_xdg_current_desktop() {
    //     clear_env();
    //     env::set_var("XDG_CURRENT_DESKTOP", "Budgie:GNOME");
    //     let result = get_de(false, None);
    //     assert_eq!(result, Some("Budgie".to_string()));
    // }

    // #[test]
    // fn detects_de_from_gnome_fallback() {
    //     clear_env();
    //     env::set_var("GNOME_DESKTOP_SESSION_ID", "this-is-gnome");
    //     let result = get_de(false, None);
    //     assert_eq!(result, Some("GNOME".to_string()));
    // }

    // #[test]
    // fn detects_de_from_mate_fallback() {
    //     clear_env();
    //     env::set_var("MATE_DESKTOP_SESSION_ID", "mate-session");
    //     let result = get_de(false, None);
    //     assert_eq!(result, Some("MATE".to_string()));
    // }

    // #[test]
    // fn detects_de_from_trinity_fallback() {
    //     clear_env(); // make sure it clears ALL related env vars
    //     env::set_var("TDE_FULL_SESSION", "true");
    //     let result = get_de(false, None);
    //     assert_eq!(result, Some("Trinity".to_string()));
    // }

    #[test]
    fn excludes_wm_that_matches_de() {
        let env_lock = clear_env();
        env_lock.set_var("DESKTOP_SESSION", "sway");
        let result = get_de(false, Some("sway"));
        assert_eq!(result, None);
        drop(env_lock);
    }

    // #[test]
    // fn tags_wayland_session() {
    //     clear_env();
    //     env::set_var("DESKTOP_SESSION", "gnome");
    //     env::set_var("WAYLAND_DISPLAY", "wayland-0");
    //     let result = get_de(false, None);
    //     assert_eq!(result, Some("GNOME (Wayland)".to_string()));
    // }

    #[test]
    fn normalize_de_variants() {
        let mut de = "xfce".to_string();
        normalize_de_name(&mut de);
        assert_eq!(de, "Xfce");

        let mut de = "Xfce4".to_string();
        normalize_de_name(&mut de);
        assert_eq!(de, "Xfce4");

        let mut de = "mate".to_string();
        normalize_de_name(&mut de);
        assert_eq!(de, "MATE");

        let mut de = "gnome".to_string();
        normalize_de_name(&mut de);
        assert_eq!(de, "GNOME");

        let mut de = "lxqt".to_string();
        normalize_de_name(&mut de);
        assert_eq!(de, "LXQt");

        let mut de = "plasma-kde".to_string();
        normalize_de_name(&mut de);
        assert_eq!(de, "Plasma");
    }
}
