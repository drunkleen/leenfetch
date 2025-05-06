use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub fn get_wm_theme(wm: &str, de: Option<&str>) -> Option<String> {
    let wm = match wm {
        "gnome-shell" | "GNOME Shell" => "Mutter",
        "kwin_x11" | "kwin_wayland" => "KWin",
        _ => wm,
    };
    match wm {
        "Xfwm4" => run_command("xfconf-query", &["-c", "xfwm4", "-p", "/general/theme"]),

        "Openbox" => {
            let ob_file = match de.unwrap_or_default() {
                d if d.starts_with("LXDE") => "lxde-rc.xml",
                d if d.starts_with("LXQt") => "lxqt-rc.xml",
                _ => "rc.xml",
            };

            let mut xdg_config_home = env::var("XDG_CONFIG_HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|_| {
                    let mut p = home_dir().unwrap_or_default();
                    p.push(".config");
                    p
                });

            xdg_config_home.push("openbox");
            xdg_config_home.push(ob_file);

            if xdg_config_home.exists() {
                let contents = fs::read_to_string(xdg_config_home).ok()?;
                for line in contents.lines() {
                    if line.contains("<name>") {
                        return line
                            .split(|c| c == '<' || c == '>')
                            .nth(2)
                            .map(|s| s.to_string());
                    }
                }
            }
            None
        }

        "Fluxbox" => {
            let mut path = home_dir()?;
            path.push(".fluxbox/init");
            if path.exists() {
                let contents = fs::read_to_string(path).ok()?;
                for line in contents.lines() {
                    if line.contains("styleFile") {
                        return line.split('/').last().map(|s| s.to_string());
                    }
                }
            }
            None
        }

        "IceWM" => {
            let mut path = home_dir()?;
            path.push(".icewm/theme");
            if path.exists() {
                let contents = fs::read_to_string(path).ok()?;
                for line in contents.lines() {
                    if !line.starts_with('#') {
                        return line
                            .split(&['"', ',', '/'][..])
                            .filter(|s| !s.is_empty())
                            .nth(1)
                            .map(|s| s.to_string());
                    }
                }
            }
            None
        }

        "KWin" | "KWinX11" | "KWinWayland" => {
            let config_dirs = vec!["~/.config/kwinrc", "~/.config/kdebugrc"];

            for config_path in config_dirs {
                let path = expand_tilde(config_path);
                if path.exists() {
                    let content = fs::read_to_string(path).ok()?;
                    for line in content.lines() {
                        if line.contains("theme=") {
                            return Some(line.trim_start_matches("theme=").to_string());
                        } else if line.contains("PluginLib=kwin3_") {
                            return Some(line.trim_start_matches("PluginLib=kwin3_").to_string());
                        } else if line.contains("library=org.kde.") {
                            return Some(line.replace("library=org.kde.", ""));
                        }
                    }
                }
            }
            None
        }

        "Mutter" | "Gala" | "Compiz" => run_command(
            "gsettings",
            &["get", "org.gnome.shell.extensions.user-theme", "name"],
        )
        .filter(|s| !s.trim_matches('\'').is_empty())
        .or_else(|| {
            run_command(
                "gsettings",
                &["get", "org.gnome.desktop.wm.preferences", "theme"],
            )
        }),

        "Cinnamon" => {
            let detheme = run_command("gsettings", &["get", "org.cinnamon.theme", "name"])?;
            let wmtheme = run_command(
                "gsettings",
                &["get", "org.cinnamon.desktop.wm.preferences", "theme"],
            )?;
            Some(format!(
                "{} ({})",
                detheme.trim_matches('\''),
                wmtheme.trim_matches('\'')
            ))
        }

        _ => None,
    }
}

fn home_dir() -> Option<PathBuf> {
    env::var("HOME").ok().map(PathBuf::from)
}

fn run_command(cmd: &str, args: &[&str]) -> Option<String> {
    Command::new(cmd)
        .args(args)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
}

fn expand_tilde(path: &str) -> PathBuf {
    if let Some(stripped) = path.strip_prefix("~/") {
        if let Some(home) = home_dir() {
            return home.join(stripped);
        }
    }
    PathBuf::from(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_wm_theme_invalid_wm() {
        assert_eq!(get_wm_theme("UnknownWM", None), None);
    }

    #[test]
    fn test_expand_tilde_resolves_home() {
        let expanded = expand_tilde("~/test-folder");
        let home = std::env::var("HOME").map(std::path::PathBuf::from).unwrap();
        assert!(expanded.starts_with(&home));
    }

    #[test]
    fn test_get_wm_theme_safe_return() {
        // Should never panic even if no config exists
        let theme = get_wm_theme("Fluxbox", None);
        assert!(theme.is_none() || !theme.as_ref().unwrap().is_empty());
    }
}
