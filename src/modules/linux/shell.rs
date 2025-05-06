use std::env;
use std::path::Path;
use std::process::Command;

/// Get the current shell name and version.
///
/// If `show_path` is true, the full path to the shell executable will be included.
/// If `show_version` is true, the version of the shell will be included.
///
/// Returns `None` if the SHELL environment variable is not set.
///
/// # Examples
///
///
pub fn get_shell(show_path: bool, show_version: bool) -> Option<String> {
    let shell_path = env::var("SHELL").ok()?;
    let shell_name = Path::new(&shell_path).file_name()?.to_string_lossy();

    let mut shell = if show_path {
        format!("{} ", shell_path)
    } else {
        format!("{} ", shell_name)
    };

    if !show_version {
        return Some(shell.trim_end().to_string());
    }

    let version = match shell_name.as_ref() {
        "bash" => {
            // Try BASH_VERSION or fallback to subprocess
            env::var("BASH_VERSION")
                .ok()
                .or_else(|| run_version_var(&shell_path, "BASH_VERSION"))
                .map(|v| v.split('-').next().unwrap_or("").to_string())
        }
        "zsh" => run_version_string(&shell_path),
        "fish" => run_version_arg(&shell_path, "--version"),
        "nu" => run_nu_version(&shell_path),
        "yash" => run_yash_version(&shell_path),
        "tcsh" => run_version_var(&shell_path, "tcsh"),
        _ => run_version_arg(&shell_path, "--version"),
    };

    if let Some(ver) = version {
        shell.push_str(&ver);
    }

    Some(clean_shell_string(shell))
}

fn run_version_var(shell_path: &str, var: &str) -> Option<String> {
    Command::new(shell_path)
        .arg("-c")
        .arg(format!("printf %s \"${}\"", var))
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
}

fn run_version_arg(shell_path: &str, arg: &str) -> Option<String> {
    Command::new(shell_path)
        .arg(arg)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| {
            let s = String::from_utf8_lossy(&o.stdout);
            s.lines().next().unwrap_or("").trim().to_string()
        })
}

fn run_version_string(shell_path: &str) -> Option<String> {
    let ver = run_version_arg(shell_path, "--version")?;
    Some(
        ver.split_whitespace()
            .find(|part| part.chars().next().unwrap_or(' ').is_numeric())
            .unwrap_or("")
            .to_string(),
    )
}

fn run_nu_version(shell_path: &str) -> Option<String> {
    Command::new(shell_path)
        .arg("-c")
        .arg("version | get version")
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
}

fn run_yash_version(shell_path: &str) -> Option<String> {
    let out = Command::new(shell_path)
        .arg("--version")
        .output()
        .ok()?
        .stdout;
    let raw = String::from_utf8_lossy(&out);
    let cleaned = raw
        .replace("yash", "")
        .replace("Yet another shell", "")
        .lines()
        .next()
        .unwrap_or("")
        .trim()
        .to_string();
    Some(cleaned)
}

fn clean_shell_string(s: String) -> String {
    s.replace(", version", "")
        .replace("xonsh/", "xonsh ")
        .replace("options", "")
        .split('(')
        .next()
        .unwrap_or("")
        .trim()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_no_version() {
        let shell = get_shell(false, false);
        assert!(shell.is_some());
        let val = shell.unwrap();
        assert!(!val.contains("--version"));
    }

    #[test]
    fn test_shell_path_on() {
        let shell = get_shell(true, false);
        assert!(shell.is_some());
        let val = shell.unwrap();
        assert!(val.starts_with("/"));
    }

    #[test]
    fn test_shell_version_optional() {
        let shell = get_shell(false, true);
        assert!(shell.is_some());
    }
}
