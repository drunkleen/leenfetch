use std::process::Command;

pub fn get_gpus() -> Vec<String> {
    let output = Command::new("wmic")
        .args(["path", "win32_VideoController", "get", "Name"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_default();

    let mut gpus = Vec::new();

    for line in output.lines() {
        let trimmed = line.trim();
        if !trimmed.is_empty() && !trimmed.eq_ignore_ascii_case("Name") {
            if !gpus.contains(&trimmed.to_string()) {
                gpus.push(trimmed.to_string());
            }
        }
    }

    if gpus.is_empty() {
        vec!["Unknown GPU".to_string()]
    } else {
        gpus
    }
}
