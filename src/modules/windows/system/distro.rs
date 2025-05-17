use std::{env, process::Command};

use crate::modules::enums::DistroDisplay;

pub fn get_distro(display: DistroDisplay) -> String {
    let name = get_product_name();
    let version = get_version();
    let arch = env::var("PROCESSOR_ARCHITECTURE").unwrap_or_else(|_| "unknown".into());
    let model = infer_windows_model(&name, &version);

    match display {
        DistroDisplay::Name => name,
        DistroDisplay::NameVersion => format!("{name} {version}").trim().to_string(),
        DistroDisplay::NameArch => format!("{name} {arch}").to_string(),
        DistroDisplay::NameModel => format!("{name} {model}").trim().to_string(),
        DistroDisplay::NameModelVersion => format!("{name} {model} {version}").trim().to_string(),
        DistroDisplay::NameModelArch => format!("{name} {model} {arch}").trim().to_string(),
        DistroDisplay::NameModelVersionArch => format!("{name} {model} {version} {arch}")
            .trim()
            .to_string(),
    }
}

fn get_product_name() -> String {
    if let Ok(output) = Command::new("wmic").args(["os", "get", "Caption"]).output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            if line.contains("Windows") {
                return line.trim().to_string();
            }
        }
    }

    "Windows".into()
}

fn get_version() -> String {
    if let Ok(output) = Command::new("ver").output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        return stdout
            .lines()
            .next()
            .unwrap_or("")
            .replace("[Version", "")
            .replace("]", "")
            .trim()
            .to_string();
    }

    "Unknown".into()
}

fn infer_windows_model(name: &str, version: &str) -> String {
    let n = name.to_lowercase();
    let v = version.to_lowercase();

    if n.contains("ltsc") || n.contains("enterprise lts") {
        return "LTS".to_string();
    }

    if n.contains("insider") {
        return "Testing".to_string();
    }

    if v.contains("dev") || v.contains("preview") {
        return "Preview".to_string();
    }

    "Stable".to_string()
}
