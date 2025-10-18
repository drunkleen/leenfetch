use std::process::Command;
use crate::modules::windows::utils::run_powershell_json;

pub fn get_model() -> Option<String> {
    // Try WMIC first
    if let Ok(output) = Command::new("wmic")
        .args(["computersystem", "get", "manufacturer,model"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if let Some(line) = stdout.lines().skip(1).find(|l| !l.trim().is_empty()) {
            let cleaned = cleanup_model_string(line);
            return Some(cleaned);
        }
    }

    // PowerShell fallback
    #[derive(serde::Deserialize)]
    struct Rec { manufacturer: Option<String>, model: Option<String> }
    let rec: Option<Rec> = run_powershell_json(
        "Get-CimInstance Win32_ComputerSystem | Select-Object -First 1 Manufacturer,Model",
    );
    let rec = rec?;
    let s = format!(
        "{} {}",
        rec.manufacturer.unwrap_or_default(),
        rec.model.unwrap_or_default()
    );
    Some(cleanup_model_string(&s))
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
