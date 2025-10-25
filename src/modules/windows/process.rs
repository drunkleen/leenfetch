use once_cell::sync::OnceCell;
use std::process::{Command, Stdio};
use winapi::shared::minwindef::DWORD;
use winapi::um::tlhelp32::{
    CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W, TH32CS_SNAPPROCESS,
};
use winapi::um::handleapi::CloseHandle;

// Cache process names (lowercased) for the duration of a single run to avoid repeated scans.
static PROCESS_NAMES: OnceCell<Vec<String>> = OnceCell::new();

/// Returns a lowercase list of running process image names (e.g., "explorer.exe").
/// Uses the Toolhelp32 snapshot API for speed and zero external processes.
pub fn process_names_lower() -> &'static [String] {
    PROCESS_NAMES.get_or_init(|| {
        if crate::modules::windows::is_safe_mode() {
            enumerate_processes_via_tasklist_lower()
        } else {
            enumerate_processes_lower()
        }
    })
}

fn enumerate_processes_lower() -> Vec<String> {
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snapshot == (-1isize) as _ {
            return Vec::new();
        }

        let mut names: Vec<String> = Vec::with_capacity(256);
        let mut entry: PROCESSENTRY32W = std::mem::zeroed();
        entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as DWORD;

        if Process32FirstW(snapshot, &mut entry) != 0 {
            loop {
                let end = entry
                    .szExeFile
                    .iter()
                    .position(|&c| c == 0)
                    .unwrap_or(entry.szExeFile.len());
                if end > 0 {
                    let name = String::from_utf16_lossy(&entry.szExeFile[..end])
                        .to_ascii_lowercase();
                    names.push(name);
                }

                if Process32NextW(snapshot, &mut entry) == 0 {
                    break;
                }
            }
        }

        CloseHandle(snapshot);

        names
    }
}

fn enumerate_processes_via_tasklist_lower() -> Vec<String> {
    let out = Command::new("tasklist")
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output();
    let Ok(output) = out else { return Vec::new() };
    if !output.status.success() { return Vec::new(); }
    let s = String::from_utf8_lossy(&output.stdout).to_lowercase();
    // Extract the image name column; simplest approach is to take token before first whitespace per line.
    let mut names = Vec::with_capacity(128);
    for line in s.lines() {
        // Skip headers/empty
        if !line.contains(".exe") { continue; }
        let first = line.split_whitespace().next().unwrap_or("");
        if !first.is_empty() { names.push(first.to_string()); }
    }
    names
}
