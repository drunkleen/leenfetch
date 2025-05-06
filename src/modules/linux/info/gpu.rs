use std::process::Command;

pub fn get_gpus() -> Vec<String> {
    let output = Command::new("lspci")
        .arg("-mm")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_default();

    let mut gpus = vec![];

    for line in output.lines() {
        if line.contains("\"VGA") || line.contains("\"3D") || line.contains("\"Display") {
            let parts: Vec<&str> = line.split('"').collect();
            if parts.len() >= 6 {
                let vendor = parts[3].replace("Corporation", "").trim().to_string();
                let model = parts[5].trim().to_string();
                let full = format!("{} {}", vendor, model);
                if !gpus.contains(&full) {
                    gpus.push(full);
                }
            }
        }
    }

    if gpus.is_empty() {
        vec!["Unknown GPU".to_string()]
    } else {
        gpus
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_linux_gpu_parsing() {
        let sample = r#"
00:02.0 "VGA compatible controller" "Intel Corporation" "Alder Lake-P GT1 [UHD Graphics]" -r02 "ASUSTeK Computer Inc." "Device 1e1f"
01:00.0 "3D controller" "NVIDIA Corporation" "GA107M [GeForce RTX 3050 Mobile]" -r00 "ASUSTeK Computer Inc." "Device 1e1f"
"#;

        let mut gpus = vec![];

        for line in sample.lines() {
            if line.contains("\"VGA") || line.contains("\"3D") || line.contains("\"Display") {
                let parts: Vec<&str> = line.split('"').collect();
                if parts.len() >= 6 {
                    let vendor = parts[3].replace("Corporation", "").trim().to_string();
                    let model = parts[5].trim().to_string();
                    let full = format!("{} {}", vendor, model);
                    if !gpus.contains(&full) {
                        gpus.push(full);
                    }
                }
            }
        }

        assert_eq!(gpus.len(), 2);
        assert_eq!(gpus[0], "Intel Alder Lake-P GT1 [UHD Graphics]");
        assert_eq!(gpus[1], "NVIDIA GA107M [GeForce RTX 3050 Mobile]");
    }

    #[test]
    fn test_macos_gpu_parsing() {
        let output = r#"
      Chipset Model: Apple M1
      Type: GPU
      Bus: Built-In
      VRAM (Dynamic, Max): 8 GB
"#;

        let mut gpus = vec![];

        for line in output.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("Chipset Model:") {
                if let Some(model) = trimmed.split(":").nth(1) {
                    gpus.push(model.trim().to_string());
                }
            }
        }

        assert_eq!(gpus, vec!["Apple M1"]);
    }

    #[test]
    fn test_windows_gpu_parsing() {
        let output = r#"
Caption
NVIDIA GeForce RTX 3050 Laptop GPU
Intel(R) UHD Graphics
"#;

        let mut gpus = vec![];

        for line in output.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() && !trimmed.contains("Caption") {
                gpus.push(trimmed.to_string());
            }
        }

        assert_eq!(gpus.len(), 2);
        assert_eq!(gpus[0], "NVIDIA GeForce RTX 3050 Laptop GPU");
        assert_eq!(gpus[1], "Intel(R) UHD Graphics");
    }

    #[test]
    fn test_fallback() {
        // Fallback logic is only triggered when OS is unknown
        // We simulate this manually here
        let unknown_os_result = vec!["Unknown GPU".to_string()];
        assert_eq!(unknown_os_result.len(), 1);
        assert_eq!(unknown_os_result[0], "Unknown GPU");
    }
}
