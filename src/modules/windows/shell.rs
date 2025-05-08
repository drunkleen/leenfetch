use std::{path::Path, process::Command};

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
    let output = Command::new("wmic")
        .args([
            "process",
            "where",
            &format!("ProcessId={}", std::process::id()),
            "get",
            "ParentProcessId",
            "/value",
        ])
        .output()
        .ok()?;

    let parent_out = String::from_utf8_lossy(&output.stdout);
    let parent_pid_line = parent_out
        .lines()
        .find(|line| line.contains("ParentProcessId="))?;

    let parent_pid = parent_pid_line.trim().split('=').nth(1)?.trim();

    // Get process executable path of parent
    let parent_output = Command::new("wmic")
        .args([
            "process",
            "where",
            &format!("ProcessId={parent_pid}"),
            "get",
            "ExecutablePath",
            "/value",
        ])
        .output()
        .ok()?;

    let parent_exe = String::from_utf8_lossy(&parent_output.stdout);
    let exe_path_line = parent_exe
        .lines()
        .find(|line| line.contains("ExecutablePath="))?;

    let path = exe_path_line.trim().strip_prefix("ExecutablePath=")?.trim();

    Some(path.to_string())
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
