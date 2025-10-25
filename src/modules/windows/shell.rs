use std::path::Path;
use crate::modules::windows::is_safe_mode;
use crate::modules::windows::process::process_names_lower;
use winapi::shared::minwindef::DWORD;
use winapi::um::handleapi::CloseHandle;
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::tlhelp32::{
    CreateToolhelp32Snapshot, PROCESSENTRY32W, Process32FirstW, Process32NextW, TH32CS_SNAPPROCESS,
};
use winapi::um::winbase::QueryFullProcessImageNameW;
use std::process::{Command as Cmd, Stdio};
use std::time::{Duration, Instant};
use winapi::um::winnt::PROCESS_QUERY_LIMITED_INFORMATION;

pub fn get_shell(show_path: bool, show_version: bool) -> Option<String> {
    let parent_shell_path = if is_safe_mode() {
        detect_shell_safe_guess()?
    } else {
        detect_parent_shell()?
    };
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
        // Avoid spawning PowerShell unless explicitly the parent shell is PowerShell.
        "powershell" => Some(ps_version()),
        "pwsh" => Some(ps_version()),
        _ => None,
    };

    if let Some(ver) = version {
        shell.push_str(&ver);
    }

    Some(clean_shell_string(shell))
}

/// Detect the parent process executable path using Toolhelp and QueryFullProcessImageNameW.
fn detect_parent_shell() -> Option<String> {
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snapshot == (-1isize) as _ {
            return None;
        }

        let current_pid = std::process::id();
        let mut entry: PROCESSENTRY32W = std::mem::zeroed();
        entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as DWORD;

        let mut parent_pid: DWORD = 0;
        if Process32FirstW(snapshot, &mut entry) != 0 {
            loop {
                if entry.th32ProcessID == current_pid {
                    parent_pid = entry.th32ParentProcessID;
                    break;
                }
                if Process32NextW(snapshot, &mut entry) == 0 {
                    break;
                }
            }
        }
        CloseHandle(snapshot);

        if parent_pid == 0 {
            return None;
        }

        let hproc = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, parent_pid);
        if hproc.is_null() {
            return None;
        }

        let mut buf: Vec<u16> = vec![0u16; 32768];
        let mut size: DWORD = buf.len() as DWORD;
        let ok = QueryFullProcessImageNameW(hproc, 0, buf.as_mut_ptr(), &mut size);
        CloseHandle(hproc);
        if ok == 0 {
            // Fallback to just the exe name from PROCESSENTRY32W (re-scan to find parent entry)
            let snapshot2 = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
            if snapshot2 == (-1isize) as _ {
                return None;
            }
            let mut e2: PROCESSENTRY32W = std::mem::zeroed();
            e2.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as DWORD;
            let mut name = None;
            if Process32FirstW(snapshot2, &mut e2) != 0 {
                loop {
                    if e2.th32ProcessID == parent_pid {
                        let end = e2
                            .szExeFile
                            .iter()
                            .position(|&c| c == 0)
                            .unwrap_or(e2.szExeFile.len());
                        name = Some(String::from_utf16_lossy(&e2.szExeFile[..end]));
                        break;
                    }
                    if Process32NextW(snapshot2, &mut e2) == 0 {
                        break;
                    }
                }
            }
            CloseHandle(snapshot2);
            return name.map(|n| n.trim().to_string());
        }

        let path = String::from_utf16_lossy(&buf[..size as usize])
            .trim()
            .to_string();
        if path.is_empty() { None } else { Some(path) }
    }
}

fn get_cmd_version() -> String {
    let output = std::process::Command::new("cmd")
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

// Lightweight PowerShell version probe: tries pwsh then powershell and returns version string.
fn ps_version() -> String {
    // Only attempt the first one found in PATH to avoid multiple spawns.
    for (exe, arg) in [("pwsh", "-Command"), ("powershell", "-Command")] {
        if let Ok(mut child) = Cmd::new(exe)
            .args([arg, "$PSVersionTable.PSVersion.ToString()"])
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
        {
            let deadline = Instant::now() + Duration::from_millis(300);
            loop {
                if let Ok(Some(_)) = child.try_wait() { break; }
                if Instant::now() > deadline { let _ = child.kill(); let _ = child.wait(); break; }
                std::thread::sleep(Duration::from_millis(10));
            }
            if let Ok(out) = child.wait_with_output() {
                if out.status.success() {
                    let s = String::from_utf8_lossy(&out.stdout);
                    if let Some(first) = s.lines().next() { return first.trim().to_string(); }
                }
            }
        }
    }
    String::new()
}

// Heuristic, AV-friendly shell detection without process introspection.
// Priority:
// 1) COMSPEC env (typical for cmd)
// 2) Presence of PowerShell env vars
// 3) Fall back to known process list via tasklist (safe-mode path only)
fn detect_shell_safe_guess() -> Option<String> {
    if let Ok(comspec) = std::env::var("ComSpec") {
        let p = Path::new(&comspec);
        if p.exists() {
            return Some(p.to_string_lossy().to_string());
        }
    }
    // Detect PowerShell via typical env hints
    if std::env::var_os("PSModulePath").is_some()
        || std::env::var_os("POWERSHELL_DISTRIBUTION_CHANNEL").is_some()
    {
        // Prefer pwsh when on PATH; otherwise generic name
        return Some("pwsh".to_string());
    }
    // Last resort: check process list names via tasklist (safe mode uses tasklist only)
    let names = process_names_lower();
    if names.iter().any(|n| n.contains("pwsh.exe")) {
        return Some("pwsh".to_string());
    }
    if names.iter().any(|n| n.contains("powershell.exe")) {
        return Some("powershell".to_string());
    }
    if names.iter().any(|n| n.contains("cmd.exe")) {
        return Some("cmd".to_string());
    }
    // Fallback
    Some("cmd".to_string())
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
