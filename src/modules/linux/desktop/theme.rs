use std::{env, fs, process::Command};

pub fn get_theme(de: Option<&str>) -> Option<String> {
    if env::var_os("DISPLAY").is_none() {
        return None;
    }

    let de = de.unwrap_or("").to_lowercase();
    let home = env::var("HOME").unwrap_or_default();

    let mut gtk2 = None;
    let mut gtk3 = None;
    let mut gtk4 = None;
    let mut kde = None;
    let mut qt = None;

    // KDE: look in kdeglobals
    if de.contains("kde") || de.contains("plasma") {
        let kde_paths = [
            format!("{home}/.config/kdeglobals"),
            "/etc/xdg/kdeglobals".to_string(),
        ];

        for path in kde_paths {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Some(line) = content.lines().find(|l| l.starts_with("Name=")) {
                    kde = Some(line.trim_start_matches("Name=").trim().to_string());
                    break;
                }
            }
        }
    }

    // GTK3: via gsettings
    if let Ok(output) = Command::new("gsettings")
        .args(["get", "org.gnome.desktop.interface", "gtk-theme"])
        .output()
    {
        if output.status.success() {
            let val = String::from_utf8_lossy(&output.stdout)
                .trim()
                .trim_matches('\'')
                .to_string();
            if !val.is_empty() {
                gtk3 = Some(val.clone());
                gtk2 = Some(val.clone()); // fallback
            }
        }
    }

    // GTK4: ~/.config/gtk-4.0/settings.ini → [Settings] gtk-theme-name
    let gtk4_path = format!("{home}/.config/gtk-4.0/settings.ini");
    if let Ok(content) = fs::read_to_string(&gtk4_path) {
        for line in content.lines() {
            if let Some(val) = line.trim().strip_prefix("gtk-theme-name=") {
                gtk4 = Some(val.trim_matches('"').to_string());
                break;
            }
        }
    }

    // GTK2: fallback to gtkrc
    if gtk2.is_none() {
        let gtk2_paths = [
            format!("{home}/.gtkrc-2.0"),
            "/etc/gtk-2.0/gtkrc".into(),
            "/usr/share/gtk-2.0/gtkrc".into(),
        ];
        for path in gtk2_paths {
            if let Ok(content) = fs::read_to_string(&path) {
                for line in content.lines() {
                    if let Some((_, val)) = line
                        .trim_start()
                        .strip_prefix("gtk-theme-name")
                        .and_then(|l| l.split_once('='))
                    {
                        gtk2 = Some(val.trim().trim_matches('"').to_string());
                        break;
                    }
                }
            }
            if gtk2.is_some() {
                break;
            }
        }
    }

    // Qt: from qt5ct / qt6ct config files
    let qt_paths = [
        format!("{home}/.config/qt5ct/qt5ct.conf"),
        format!("{home}/.config/qt6ct/qt6ct.conf"),
    ];
    for path in qt_paths {
        if let Ok(content) = fs::read_to_string(&path) {
            for line in content.lines() {
                if let Some(val) = line.trim().strip_prefix("style=") {
                    qt = Some(val.trim().to_string());
                    break;
                }
            }
        }
        if qt.is_some() {
            break;
        }
    }

    // Compose final output
    let mut result = Vec::new();

    if let Some(val) = kde {
        result.push(format!("{val} [KDE]"));
    }

    if let Some(val) = qt {
        result.push(format!("{val} [Qt]"));
    }

    match (&gtk2, &gtk3) {
        (Some(g2), Some(g3)) if g2 == g3 => result.push(format!("{g3} [GTK2/3]")),
        (Some(g2), Some(g3)) => {
            result.push(format!("{g2} [GTK2]"));
            result.push(format!("{g3} [GTK3]"));
        }
        (Some(g2), None) => result.push(format!("{g2} [GTK2]")),
        (None, Some(g3)) => result.push(format!("{g3} [GTK3]")),
        _ => {}
    }

    if let Some(val) = gtk4 {
        result.push(format!("{val} [GTK4]"));
    }

    if result.is_empty() {
        None
    } else {
        Some(result.join(", "))
    }
}
