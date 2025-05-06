use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn get_theme(de: Option<&str>) -> Option<String> {
    if env::var("DISPLAY").is_err() {
        return None;
    }

    let de = de.unwrap_or("").to_lowercase();
    let mut gtk2_theme = None;
    let mut gtk3_theme = None;
    let mut kde_theme = None;

    // KDE config
    if de.contains("kde") || de.contains("plasma") {
        let kdeglobals_paths = vec![
            format!(
                "{}/.config/kdeglobals",
                env::var("HOME").unwrap_or_default()
            ),
            "/etc/xdg/kdeglobals".to_string(),
        ];

        for path in kdeglobals_paths {
            let path = Path::new(&path);
            if path.exists() {
                if let Ok(content) = fs::read_to_string(path) {
                    for line in content.lines() {
                        if line.starts_with("Name=") {
                            kde_theme = Some(line.trim_start_matches("Name=").to_string());
                            break;
                        }
                    }
                }
                break;
            }
        }
    }

    // GTK3 theme
    if let Ok(out) = Command::new("gsettings")
        .args(["get", "org.gnome.desktop.interface", "gtk-theme"])
        .output()
    {
        let val = String::from_utf8_lossy(&out.stdout)
            .trim()
            .trim_matches('\'')
            .to_string();
        if !val.is_empty() && !val.contains("No such schema") {
            gtk3_theme = Some(val.clone());
            gtk2_theme = Some(val); // Assume GTK2 = GTK3 if nothing else found
        }
    }

    // GTK2 fallback
    if gtk2_theme.is_none() {
        let rc_paths = vec![
            format!("{}/.gtkrc-2.0", env::var("HOME").unwrap_or_default()),
            "/etc/gtk-2.0/gtkrc".to_string(),
            "/usr/share/gtk-2.0/gtkrc".to_string(),
        ];

        for path in rc_paths {
            let path = Path::new(&path);
            if path.exists() {
                if let Ok(content) = fs::read_to_string(path) {
                    for line in content.lines() {
                        if line.trim_start().starts_with("gtk-theme-name") {
                            if let Some((_, val)) = line.split_once('=') {
                                gtk2_theme = Some(val.trim_matches('"').trim().to_string());
                            }
                            break;
                        }
                    }
                }
            }
        }
    }

    // Final formatting
    let mut components = Vec::new();

    if let Some(k) = kde_theme {
        components.push(format!("{k} [KDE]"));
    }

    match (gtk2_theme.as_ref(), gtk3_theme.as_ref()) {
        (Some(g2), Some(g3)) if g2 == g3 => components.push(format!("{g3} [GTK2/3]")),
        (Some(g2), Some(g3)) => {
            components.push(format!("{g2} [GTK2]"));
            components.push(format!("{g3} [GTK3]"));
        }
        (Some(g2), None) => components.push(format!("{g2} [GTK2]")),
        (None, Some(g3)) => components.push(format!("{g3} [GTK3]")),
        _ => {}
    }

    if components.is_empty() {
        None
    } else {
        Some(components.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_safe() {
        let result = get_theme(Some("gnome"));
        if let Some(theme) = result {
            assert!(theme.contains("GTK") || theme.contains("KDE"));
        }
    }
}
