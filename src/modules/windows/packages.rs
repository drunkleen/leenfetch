use std::env;
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use crate::modules::enums::PackageShorthand;

pub fn get_packages(shorthand: PackageShorthand) -> Option<String> {
    // If user requested "off", do not scan any managers.
    if shorthand == PackageShorthand::Off {
        return None;
    }

    let mut total = 0u64;
    let mut managers = vec![]; // detailed entries for On mode
    let mut names = vec![];    // just names for Tiny mode

    // Prefer fast filesystem counts for choco/scoop
    if let Some(count) = count_choco_fs() {
        total += count;
        managers.push(format!("{count} (choco)"));
        names.push("choco");
    } else if is_installed("choco") {
        if let Some(count) = count_lines_timeout(
            Command::new("choco").args(["list", "--local-only", "--no-color"]),
            Duration::from_millis(600),
        ) {
            total += count;
            managers.push(format!("{count} (choco)"));
            names.push("choco");
        } else {
            names.push("choco");
            managers.push("choco".to_string());
        }
    }

    if let Some(count) = count_scoop_fs() {
        total += count;
        managers.push(format!("{count} (scoop)"));
        names.push("scoop");
    } else if is_installed("scoop") {
        if let Some(count) = count_lines_timeout(Command::new("scoop").arg("list"), Duration::from_millis(400)) {
            total += count;
            managers.push(format!("{count} (scoop)"));
            names.push("scoop");
        } else {
            names.push("scoop");
            managers.push("scoop".to_string());
        }
    }

    // winget is frequently slow; try briefly and otherwise only record presence
    if is_installed("winget") {
        if let Some(count) = count_lines_timeout(Command::new("winget").arg("list"), Duration::from_millis(1000)) {
            total += count;
            managers.push(format!("{count} (winget)"));
            names.push("winget");
        } else {
            names.push("winget");
            managers.push("winget".to_string());
        }
    }

    if names.is_empty() {
        return None;
    }

    match shorthand {
        PackageShorthand::On => Some(managers.join(", ")),
        PackageShorthand::Tiny => Some(format!("{total} ({})", names.join(", "))),
        PackageShorthand::Off => None,
    }
}

fn is_installed(cmd: &str) -> bool {
    // Avoid spawning external processes; search PATH with PATHEXT
    let path = match env::var_os("PATH") { Some(p) => p, None => return false };
    let pathext = env::var_os("PATHEXT").unwrap_or_else(|| ".COM;.EXE;.BAT;.CMD".into());
    let exts: Vec<String> = pathext
        .to_string_lossy()
        .split(';')
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().to_string())
        .collect();
    for dir in env::split_paths(&path) {
        if dir.as_os_str().is_empty() { continue; }
        // Try exact filename (in case caller provided full name with extension)
        let candidate = dir.join(cmd);
        if candidate.is_file() { return true; }
        // Try with PATHEXT suffixes
        for ext in &exts {
            let mut name = cmd.to_string();
            // Ensure a dot-prefixed ext
            if !ext.starts_with('.') {
                name.push('.');
            }
            name.push_str(ext.trim_start_matches('.'));
            let c = dir.join(&name);
            if c.is_file() { return true; }
        }
    }
    false
}

fn count_choco_fs() -> Option<u64> {
    let programdata = env::var_os("PROGRAMDATA").unwrap_or_else(|| PathBuf::from(r"C:\\ProgramData").into());
    let lib = PathBuf::from(programdata).join("chocolatey").join("lib");
    let entries = fs::read_dir(&lib).ok()?;
    let mut count = 0u64;
    for e in entries.flatten() {
        let path = e.path();
        if path.is_dir() {
            count += 1;
        }
    }
    Some(count)
}

fn count_scoop_fs() -> Option<u64> {
    let root = env::var_os("SCOOP").map(PathBuf::from).or_else(|| {
        env::var_os("USERPROFILE").map(|u| PathBuf::from(u).join("scoop"))
    })?;
    let apps = root.join("apps");
    let entries = fs::read_dir(&apps).ok()?;
    let mut count = 0u64;
    for e in entries.flatten() {
        let p = e.path();
        if p.is_dir() {
            let name = p.file_name().and_then(|s| s.to_str()).unwrap_or("");
            if name.eq_ignore_ascii_case("scoop") { continue; }
            count += 1;
        }
    }
    Some(count)
}

fn count_lines_timeout(cmd: &mut Command, timeout: Duration) -> Option<u64> {
    cmd.stdout(Stdio::piped()).stderr(Stdio::null());
    let mut child = cmd.spawn().ok()?;
    let start = Instant::now();
    loop {
        if let Some(_status) = child.try_wait().ok()? {
            break;
        }
        if start.elapsed() > timeout {
            // Kill the process if it runs too long
            let _ = child.kill();
            let _ = child.wait();
            return None;
        }
        std::thread::sleep(Duration::from_millis(10));
    }

    let mut out = String::new();
    if let Some(mut stdout) = child.stdout.take() {
        let _ = stdout.read_to_string(&mut out);
    }
    // Ensure process fully reaped
    let _ = child.wait();

    if out.is_empty() {
        return None;
    }
    let mut lines = out.lines().filter(|l| !l.trim().is_empty()).count() as u64;
    // choco has a footer line, so subtract 1
    if cmd.get_program().to_string_lossy().contains("choco") && lines > 0 {
        lines = lines.saturating_sub(1);
    }
    Some(lines)
}
