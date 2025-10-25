use crate::modules::windows::process::process_names_lower;

pub fn get_wm() -> Option<String> {
    let names = process_names_lower();
    for &wm in KNOWN_WMS {
        let needle = wm;
        if names.iter().any(|p| p.contains(needle)) {
            return Some(normalize_wm(wm));
        }
    }
    None
}

fn normalize_wm(proc_name: &str) -> String {
    match proc_name.to_lowercase().as_str() {
        "dwm.exe" => "DWM".to_string(),
        "komorebi.exe" => "Komorebi".to_string(),
        "bugn.exe" | "bug.n.exe" => "bug.n".to_string(),
        "swm.exe" => "SWM".to_string(),
        "bblean.exe" => "BBLean".to_string(),
        "e17.exe" => "Enlightenment".to_string(),
        "dexpot.exe" => "Dexpot".to_string(),
        "aqua.exe" => "AquaSnap".to_string(),
        "workspacer.exe" => "Workspacer".to_string(),
        "xoblite.exe" => "xoblite".to_string(),
        "explorer.exe" => "Explorer".to_string(), // fallback shell/wm
        other => other.trim_end_matches(".exe").to_string(),
    }
}

// Known Windows WM processes
static KNOWN_WMS: &[&str] = &[
    "dwm.exe",
    "komorebi.exe",
    "bugn.exe",
    "bug.n.exe",
    "swm.exe",
    "bblean.exe",
    "e17.exe",
    "dexpot.exe",
    "aqua.exe",
    "workspacer.exe",
    "xoblite.exe",
    "explorer.exe", // Explorer acts as the shell + wm if no replacement
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_known_wms() {
        assert_eq!(normalize_wm("dwm.exe"), "DWM");
        assert_eq!(normalize_wm("komorebi.exe"), "Komorebi");
        assert_eq!(normalize_wm("bug.n.exe"), "bug.n");
        assert_eq!(normalize_wm("explorer.exe"), "Explorer");
    }

    #[test]
    fn test_get_wm_executes() {
        // Just ensures no panic
        let _ = get_wm();
    }
}
