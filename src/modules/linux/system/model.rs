use std::fs;

pub fn get_model() -> Option<String> {
    // Try Android first
    if std::path::Path::new("/system/app/").exists()
        && std::path::Path::new("/system/priv-app/").exists()
    {
        return Some("Android Device".to_string()); // Placeholder: no getprop support in pure Rust
    }

    let paths = [
        (
            "/sys/devices/virtual/dmi/id/board_vendor",
            "/sys/devices/virtual/dmi/id/board_name",
        ),
        (
            "/sys/devices/virtual/dmi/id/product_name",
            "/sys/devices/virtual/dmi/id/product_version",
        ),
        ("/sys/firmware/devicetree/base/model", ""),
        ("/tmp/sysinfo/model", ""),
    ];

    let mut model = String::new();

    for (file1, file2) in paths {
        if std::path::Path::new(file1).exists() {
            model.push_str(&read_first_line(file1).unwrap_or_default());
            if !file2.is_empty() && std::path::Path::new(file2).exists() {
                model.push(' ');
                model.push_str(&read_first_line(file2).unwrap_or_default());
            }
            break;
        }
    }

    if model.is_empty() {
        return None;
    }

    let cleaned = cleanup_model_string(&model);
    Some(cleaned)
}

// Read first line of a file
fn read_first_line<P: AsRef<std::path::Path>>(path: P) -> Option<String> {
    fs::read_to_string(path)
        .ok()
        .and_then(|s| s.lines().next().map(|line| line.trim().to_string()))
}

// Clean up known garbage strings
fn cleanup_model_string(model: &str) -> String {
    let mut s = model.to_string();

    let garbage = [
        "To be filled by O.E.M.",
        "To Be Filled",
        "OEM",
        "Not Applicable",
        "System Product Name",
        "System Version",
        "Undefined",
        "Default string",
        "Not Specified",
        "Type1ProductConfigId",
        "INVALID",
        "All Series",
        "�",
    ];

    for g in garbage {
        s = s.replace(g, "").trim().to_string();
    }

    if s.starts_with("Standard PC") {
        s = format!("KVM/QEMU ({})", model.trim());
    } else if s.starts_with("OpenBSD") {
        s = format!("vmm ({})", model.trim());
    }

    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cleanup_garbage() {
        let raw = "To be filled by O.E.M.";
        let cleaned = cleanup_model_string(raw);
        assert_eq!(cleaned, "");

        let raw = "Standard PC (i440FX)";
        let cleaned = cleanup_model_string(raw);
        assert!(cleaned.contains("KVM/QEMU"));
    }

    #[test]
    fn test_model_maybe_exists() {
        let model = get_model();
        assert!(model.is_none() || !model.as_ref().unwrap().is_empty());
    }
}
