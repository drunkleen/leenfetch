use std::process::Command;

pub fn get_model() -> Option<String> {
    let output = Command::new("wmic")
        .args(["computersystem", "get", "manufacturer,model"])
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Find the line after the header row (skip the first)
    let mut lines = stdout.lines().skip(1);

    if let Some(line) = lines.next() {
        let cleaned = cleanup_model_string(line);
        return Some(cleaned);
    }

    None
}

fn cleanup_model_string(model: &str) -> String {
    let mut s = model.trim().to_string();

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
    ];

    for g in garbage {
        s = s.replace(g, "").trim().to_string();
    }

    if s.starts_with("Standard PC") {
        return format!("KVM/QEMU ({})", model.trim());
    }

    if s.starts_with("OpenBSD") {
        return format!("vmm ({})", model.trim());
    }

    if s.is_empty() {
        return "Unknown".into();
    }

    s
}
