use std::process::Command;

/// Run a short PowerShell command and return stdout as String.
/// Tries `pwsh` first, then legacy `powershell`.
pub fn run_powershell(cmd: &str) -> Option<String> {
    // Try PowerShell Core (pwsh) first
    for exe in ["pwsh", "powershell"] {
        let output = Command::new(exe)
            .args(["-NoProfile", "-NonInteractive", "-Command", cmd])
            .output();

        match output {
            Ok(out) if out.status.success() => {
                let s = String::from_utf8_lossy(&out.stdout).to_string();
                return Some(s);
            }
            _ => continue,
        }
    }
    None
}

/// Run a PowerShell pipeline that emits JSON and parse it.
pub fn run_powershell_json<T: serde::de::DeserializeOwned>(cmd: &str) -> Option<T> {
    let full = format!("({cmd}) | ConvertTo-Json -Depth 4");
    let out = run_powershell(&full)?;
    // Some PS versions emit UTF-16 LE for certain commands; detect BOM quickly
    if out.is_empty() {
        return None;
    }
    serde_json::from_str::<T>(out.trim()).ok()
}
