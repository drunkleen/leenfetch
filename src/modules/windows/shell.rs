use std::{path::Path, process::Command};
use crate::modules::windows::utils::run_powershell;

pub fn get_shell(show_path: bool, show_version: bool) -> Option<String> {
    let parent_shell_path = detect_parent_shell()?;
    let shell_name = Path::new(&parent_shell_path)
        .file_stem()?
        .to_string_lossy()
        .to_ascii_lowercase();

    let mut shell = if show_path {
        format!("{parent_shell_path} ")
    } else {
        format!("{shell_name} ")
    };

    if !show_version {
        return Some(shell.trim_end().to_string());
    }

    let version = match shell_name.as_str() {
        "cmd" => Some(get_cmd_version()),
        "powershell" => run_version_arg(
            "powershell",
            "-Command",
            "$PSVersionTable.PSVersion.ToString()",
        ),
        "pwsh" => run_version_arg("pwsh", "-Command", "$PSVersionTable.PSVersion.ToString()"),
        "nu" => run_version_arg("nu", "--version", ""),
        "bash" => run_version_arg("bash", "--version", ""),
        _ => None,
    };

    if let Some(ver) = version {
        shell.push_str(&ver);
    }

    Some(clean_shell_string(shell))
}

/// Tries to detect the actual shell that launched this process.
fn detect_parent_shell() -> Option<String> {
    // Try WMIC first
    if let Ok(output) = Command::new("wmic").args([
        "process",
        "where",
        &format!("ProcessId={}", std::process::id()),
        "get",
        "ParentProcessId",
        "/value",
    ]).output() {
        let parent_out = String::from_utf8_lossy(&output.stdout);
        if let Some(parent_pid_line) = parent_out.lines().find(|line| line.contains("ParentProcessId=")) {
            if let Some(parent_pid) = parent_pid_line.trim().split('=').nth(1) {
                if let Ok(parent_output) = Command::new("wmic").args([
                    "process","where",&format!("ProcessId={}", parent_pid.trim()),"get","ExecutablePath","/value"
                ]).output() {
                    let parent_exe = String::from_utf8_lossy(&parent_output.stdout);
                    if let Some(exe_path_line) = parent_exe.lines().find(|line| line.contains("ExecutablePath=")) {
                        if let Some(path) = exe_path_line.trim().strip_prefix("ExecutablePath=") {
                            return Some(path.trim().to_string());
                        }
                    }
                }
            }
        }
    }

    // PowerShell CIM fallback
    let ps = run_powershell(&format!(
        "$ppid = (Get-CimInstance Win32_Process -Filter 'ProcessId={pid}').ParentProcessId; \
        (Get-CimInstance Win32_Process -Filter \"ProcessId=$ppid\").ExecutablePath",
        pid = std::process::id()
    ))?;
    let line = ps.lines().find(|l| !l.trim().is_empty())?;
    Some(line.trim().to_string())
}

fn get_cmd_version() -> String {
    let output = Command::new("cmd")
        .args(["/C", "ver"])
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .unwrap_or_default();

    output
        .lines()
        .find(|line| line.contains("Microsoft"))
        .unwrap_or("cmd")
        .replace("[Version", "")
        .replace("]", "")
        .trim()
        .to_string()
}

fn run_version_arg(shell: &str, arg1: &str, arg2: &str) -> Option<String> {
    let output = Command::new(shell).args([arg1, arg2]).output().ok()?;

    if !output.status.success() {
        return None;
    }

    let out = String::from_utf8_lossy(&output.stdout);
    Some(out.lines().next().unwrap_or("").trim().to_string())
}

fn clean_shell_string(s: String) -> String {
    s.replace(", version", "")
        .replace("Microsoft ", "")
        .replace("Windows ", "")
        .replace("options", "")
        .split('(')
        .next()
        .unwrap_or("")
        .trim()
        .to_string()
}
