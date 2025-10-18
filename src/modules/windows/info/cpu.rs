use std::process::Command;
use crate::modules::windows::utils::run_powershell;

pub fn get_cpu(
    cpu_brand: bool,
    show_speed: bool,
    show_cores: bool,
    show_temp: bool,
    speed_shorthand: bool,
    temp_unit: Option<char>,
) -> Option<String> {
    let mut output = get_cpu_model(cpu_brand);

    if show_cores {
        let cores = get_core_count();
        output = format!("{} ({})", output, cores);
    }

    if show_speed {
        if let Some(mhz) = get_cpu_speed_mhz() {
            let formatted = if mhz < 1000 {
                format!("{}MHz", mhz)
            } else {
                let mut ghz = mhz as f32 / 1000.0;
                if speed_shorthand {
                    ghz = (ghz * 10.0).round() / 10.0;
                }
                format!("{:.1}GHz", ghz)
            };
            output = format!("{} @ {}", output, formatted);
        }
    }

    if show_temp {
        if let Some(mut celsius) = get_cpu_temperature() {
            if let Some('F') = temp_unit {
                celsius = celsius * 9.0 / 5.0 + 32.0;
            }
            output = format!("{} [{:.1}°{}]", output, celsius, temp_unit.unwrap_or('C'));
        }
    }

    Some(output)
}

fn get_cpu_model(show_brand: bool) -> String {
    // Try WMIC first (older systems)
    if let Ok(output) = Command::new("wmic").args(["cpu", "get", "Name"]).output() {
        let raw = String::from_utf8_lossy(&output.stdout);
        if let Some(line) = raw.lines().skip(1).find(|l| !l.trim().is_empty()) {
            return sanitize_cpu_model(line.trim(), show_brand);
        }
    }

    // Fallback to PowerShell CIM on modern Windows (WMIC removed)
    if let Some(out) = run_powershell(
        "Get-CimInstance Win32_Processor | Select-Object -First 1 -ExpandProperty Name",
    ) {
        let line = out.lines().find(|l| !l.trim().is_empty());
        if let Some(name) = line {
            return sanitize_cpu_model(name.trim(), show_brand);
        }
    }

    "Unknown CPU".to_string()
}

fn get_core_count() -> u32 {
    // Try WMIC
    if let Ok(out) = Command::new("wmic").args(["cpu", "get", "NumberOfCores"]).output() {
        let text = String::from_utf8_lossy(&out.stdout);
        if let Some(line) = text.lines().skip(1).find(|l| !l.trim().is_empty()) {
            if let Ok(val) = line.trim().parse::<u32>() {
                return val;
            }
        }
    }

    // Fallback to PowerShell (sum across packages)
    if let Some(out) = run_powershell(
        "(Get-CimInstance Win32_Processor | Measure-Object -Property NumberOfCores -Sum).Sum",
    ) {
        if let Ok(v) = out.trim().parse::<u32>() {
            return v;
        }
    }

    1
}

fn get_cpu_speed_mhz() -> Option<u32> {
    // WMIC first
    if let Ok(output) = Command::new("wmic").args(["cpu", "get", "MaxClockSpeed"]).output() {
        let raw = String::from_utf8_lossy(&output.stdout);
        if let Some(val) = raw
            .lines()
            .skip(1)
            .find(|l| !l.trim().is_empty())
            .and_then(|s| s.trim().parse::<u32>().ok())
        {
            return Some(val);
        }
    }

    // PowerShell fallback
    if let Some(out) = run_powershell(
        "Get-CimInstance Win32_Processor | Select-Object -First 1 -ExpandProperty MaxClockSpeed",
    ) {
        return out.trim().parse::<u32>().ok();
    }

    None
}

fn get_cpu_temperature() -> Option<f32> {
    // WMIC (legacy)
    if let Ok(output) = Command::new("wmic").args([
        "/namespace:\\root\\wmi",
        "path",
        "MSAcpi_ThermalZoneTemperature",
        "get",
        "CurrentTemperature",
    ]).output()
    {
        let raw = String::from_utf8_lossy(&output.stdout);
        if let Some(raw_line) = raw
            .lines()
            .skip(1)
            .find(|l| l.trim().parse::<f32>().is_ok())
        {
            if let Ok(v) = raw_line.trim().parse::<f32>() {
                return Some((v - 2732.0) / 10.0);
            }
        }
    }

    // PowerShell CIM fallback
    if let Some(out) = run_powershell(
        "Get-CimInstance -Namespace root/\"wmi\" MSAcpi_ThermalZoneTemperature | Select-Object -First 1 -ExpandProperty CurrentTemperature",
    ) {
        if let Ok(v) = out.trim().parse::<f32>() {
            return Some((v - 2732.0) / 10.0);
        }
    }

    None
}

fn sanitize_cpu_model(model: &str, show_brand: bool) -> String {
    let mut s = model.to_string();

    let remove_patterns = [
        "(TM)",
        "(tm)",
        "(R)",
        "(r)",
        "CPU",
        "Processor",
        "with Radeon",
        "Technologies, Inc",
        "Core2",
        "Chip Revision",
        "Compute Cores",
        "FPU",
    ];

    for pat in remove_patterns.iter() {
        s = s.replace(pat, "");
    }

    if !show_brand {
        let brands = ["AMD ", "Intel ", "Qualcomm ", "Apple "];
        for b in brands {
            s = s.replacen(b, "", 1);
        }
    }

    s = s
        .split_whitespace()
        .filter(|word| {
            if let Some(stripped) = word.strip_suffix("-Core") {
                return !stripped.chars().all(|c| c.is_ascii_digit());
            }
            true
        })
        .collect::<Vec<_>>()
        .join(" ");

    s.trim().to_string()
}
