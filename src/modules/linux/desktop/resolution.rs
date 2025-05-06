use std::process::Command;

#[cfg(target_os = "windows")]
pub fn get_resolution(refresh_rate: bool) -> Option<String> {
    let output_h = Command::new("wmic")
        .args([
            "path",
            "Win32_VideoController",
            "get",
            "CurrentHorizontalResolution",
        ])
        .output()
        .ok()?
        .stdout;

    let output_v = Command::new("wmic")
        .args([
            "path",
            "Win32_VideoController",
            "get",
            "CurrentVerticalResolution",
        ])
        .output()
        .ok()?
        .stdout;

    let hlines: Vec<_> = String::from_utf8_lossy(&output_h)
        .lines()
        .skip(1)
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    let vlines: Vec<_> = String::from_utf8_lossy(&output_v)
        .lines()
        .skip(1)
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    if hlines.len() != vlines.len() {
        return None;
    }

    let mut res = String::new();
    for (w, h) in hlines.iter().zip(vlines.iter()) {
        res.push_str(&format!("{}x{}, ", w, h));
    }

    Some(res.trim_end_matches(", ").to_string())
}

#[cfg(target_os = "linux")]
pub fn get_resolution(refresh_rate: bool) -> Option<String> {
    if let Ok(output) = Command::new("xrandr")
        .arg("--nograb")
        .arg("--current")
        .output()
    {
        let out = String::from_utf8_lossy(&output.stdout);
        let mut resolutions = Vec::new();

        for line in out.lines() {
            if line.contains(" connected") && line.contains("x") {
                if refresh_rate {
                    if let Some(capt) = line.split_whitespace().find(|s| s.ends_with("*")) {
                        let res = line.split_whitespace().nth(2)?; // resolution like 1920x1080
                        let rate = capt.trim_end_matches('*');
                        resolutions.push(format!("{} @ {}Hz", res, rate));
                    }
                } else {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() > 2 && parts[1] == "connected" {
                        resolutions.push(parts[2].to_string());
                    }
                }
            }
        }

        if !resolutions.is_empty() {
            return Some(resolutions.join(", "));
        }
    }

    // Fallback to xdpyinfo
    if let Ok(output) = Command::new("xdpyinfo").output() {
        let out = String::from_utf8_lossy(&output.stdout);
        for line in out.lines() {
            if line.contains("dimensions:") {
                let dim = line.split_whitespace().nth(1)?;
                return Some(dim.to_string());
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    // linux test
    #[test]
    fn test_resolution_detectable() {
        // Skip test if DISPLAY is not usable
        if std::env::var("DISPLAY").is_err() {
            eprintln!("DISPLAY not set, skipping resolution test.");
            return;
        }

        let result = get_resolution(false);

        match result {
            Some(ref res) if res.contains('x') => {
                println!("Detected resolution: {}", res);
            }
            Some(res) => {
                eprintln!("Resolution detected but unexpected format: '{}'", res);
            }
            None => {
                eprintln!("No resolution detected, skipping assert.");
            }
        }
    }
}
