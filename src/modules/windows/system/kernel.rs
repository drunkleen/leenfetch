use crate::modules::windows::utils::run_powershell;
use std::process::Command;

#[inline(always)]
pub fn get_kernel() -> Option<String> {
    if let Ok(output) = Command::new("wmic").args(["os", "get", "Version"]).output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines().skip(1) {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                return Some(trimmed.to_string());
            }
        }
    }

    if let Some(ps) = run_powershell("(Get-CimInstance Win32_OperatingSystem).Version") {
        let s = ps.trim();
        if !s.is_empty() {
            return Some(s.to_string());
        }
    }

    None
}
