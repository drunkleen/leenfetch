use std::process::Command;

#[inline(always)]
pub fn get_kernel() -> Option<String> {
    let output = Command::new("wmic")
        .args(["os", "get", "Version"])
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines().skip(1) {
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            return Some(trimmed.to_string());
        }
    }

    None
}
