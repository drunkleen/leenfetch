use std::process::Command;

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
    let output = Command::new("wmic").args(["cpu", "get", "Name"]).output();

    if let Ok(output) = output {
        let raw = String::from_utf8_lossy(&output.stdout);
        if let Some(line) = raw.lines().skip(1).find(|l| !l.trim().is_empty()) {
            return sanitize_cpu_model(line.trim(), show_brand);
        }
    }

    "Unknown CPU".into()
}

fn get_core_count() -> u32 {
    // fallback to WMIC if num_cpus crate is not allowed
    let output = Command::new("wmic")
        .args(["cpu", "get", "NumberOfCores"])
        .output();

    if let Ok(out) = output {
        let text = String::from_utf8_lossy(&out.stdout);
        if let Some(line) = text.lines().skip(1).find(|l| !l.trim().is_empty()) {
            if let Ok(val) = line.trim().parse::<u32>() {
                return val;
            }
        }
    }

    1 // fallback
}

fn get_cpu_speed_mhz() -> Option<u32> {
    let output = Command::new("wmic")
        .args(["cpu", "get", "MaxClockSpeed"])
        .output()
        .ok()?;

    let raw = String::from_utf8_lossy(&output.stdout);
    raw.lines()
        .skip(1)
        .find(|l| !l.trim().is_empty())?
        .trim()
        .parse::<u32>()
        .ok()
}

fn get_cpu_temperature() -> Option<f32> {
    let output = Command::new("wmic")
        .args([
            "/namespace:\\\\root\\wmi",
            "path",
            "MSAcpi_ThermalZoneTemperature",
            "get",
            "CurrentTemperature",
        ])
        .output()
        .ok()?;

    let raw = String::from_utf8_lossy(&output.stdout);
    let line = raw
        .lines()
        .skip(1)
        .find(|l| l.trim().parse::<f32>().is_ok())?;
    let raw_temp = line.trim().parse::<f32>().ok()?;

    Some((raw_temp - 2732.0) / 10.0) // deciKelvin to Celsius
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
