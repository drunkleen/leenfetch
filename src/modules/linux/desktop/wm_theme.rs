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
            let path = home_dir()?.join(".config/openbox").join(ob_file);
            if path.exists() {
                let content = fs::read_to_string(path).ok()?;
                for line in content.lines() {
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
            let path = home_dir()?.join(".fluxbox/init");
            if path.exists() {
                let content = fs::read_to_string(path).ok()?;
                for line in content.lines() {
                    if line.contains("styleFile") {
                        return line.split('/').last().map(|s| s.to_string());
                    }
                }
            }
            None
        }

        "IceWM" => {
            let path = home_dir()?.join(".icewm/theme");
            if path.exists() {
                let content = fs::read_to_string(path).ok()?;
                for line in content.lines() {
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

        "KWin" => {
            let paths = ["~/.config/kwinrc", "~/.config/kdebugrc"];

            for path in paths {
                let full_path = expand_tilde(path);
                if full_path.exists() {
                    let content = fs::read_to_string(full_path).ok()?;
                    for line in content.lines() {
                        if line.contains("theme=") {
                            return Some(line.trim_start_matches("theme=").trim().to_string());
                        } else if line.contains("PluginLib=kwin3_") {
                            return Some(line.trim_start_matches("PluginLib=kwin3_").to_string());
                        } else if line.contains("library=org.kde.") {
                            return Some(line.replace("library=org.kde.", "").trim().to_string());
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
            let de_theme = run_command("gsettings", &["get", "org.cinnamon.theme", "name"])?;
            let wm_theme = run_command(
                "gsettings",
                &["get", "org.cinnamon.desktop.wm.preferences", "theme"],
            )?;
            Some(format!(
                "{} ({})",
                de_theme.trim_matches('\''),
                wm_theme.trim_matches('\'')
            ))
        }

        _ => None,
    }
}

fn run_command(cmd: &str, args: &[&str]) -> Option<String> {
    Command::new(cmd)
        .args(args)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
}

fn home_dir() -> Option<PathBuf> {
    env::var("HOME").ok().map(PathBuf::from)
}

fn expand_tilde(path: &str) -> PathBuf {
    if let Some(stripped) = path.strip_prefix("~/") {
        if let Some(home) = home_dir() {
            return home.join(stripped);
        }
    }
    PathBuf::from(path)
}
