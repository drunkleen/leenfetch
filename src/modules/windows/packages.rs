use std::process::Command;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PackageShorthand {
    Off,
    On,
    Tiny,
}

impl FromStr for PackageShorthand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        match s.to_lowercase().as_str() {
            "off" => Ok(PackageShorthand::Off),
            "on" => Ok(PackageShorthand::On),
            "tiny" => Ok(PackageShorthand::Tiny),
            _ => Err(()),
        }
    }
}

pub fn get_packages(shorthand: PackageShorthand) -> Option<String> {
    let mut total = 0u64;
    let mut managers = vec![];
    let mut names = vec![];

    if is_installed("winget") {
        if let Some(count) = count_lines(Command::new("winget").arg("list")) {
            total += count;
            managers.push(format!("{count} (winget)"));
            names.push("winget");
        }
    }

    if is_installed("choco") {
        if let Some(count) =
            count_lines(Command::new("choco").args(["list", "--local-only", "--no-color"]))
        {
            total += count;
            managers.push(format!("{count} (choco)"));
            names.push("choco");
        }
    }

    if is_installed("scoop") {
        if let Some(count) = count_lines(Command::new("scoop").arg("list")) {
            total += count;
            managers.push(format!("{count} (scoop)"));
            names.push("scoop");
        }
    }

    if total == 0 {
        return None;
    }

    match shorthand {
        PackageShorthand::Off => Some(format!("{total} total")),
        PackageShorthand::On => Some(managers.join(", ")),
        PackageShorthand::Tiny => Some(format!("{total} ({})", names.join(", "))),
    }
}

fn is_installed(cmd: &str) -> bool {
    Command::new("where")
        .arg(cmd)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn count_lines(cmd: &mut Command) -> Option<u64> {
    let output = cmd.output().ok()?;
    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines = stdout.lines().filter(|l| !l.trim().is_empty()).count() as u64;

    // choco has a footer line, so subtract 1
    if cmd.get_program().to_string_lossy().contains("choco") && lines > 0 {
        Some(lines.saturating_sub(1))
    } else {
        Some(lines)
    }
}
