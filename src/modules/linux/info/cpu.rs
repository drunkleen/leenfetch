use std::fs;
use std::path::Path;

/// Gets the CPU model, number of cores, speed, and temperature.
///
/// CPU model is sanitized to remove generic brand prefixes if `cpu_brand` is
/// false. The number of cores is only included if `show_cores` is true. The
/// speed is only included if `show_speed` is true. The temperature is only
/// included if `show_temp` is true.
///
/// The speed is formatted as "XMHz" if it is less than 1000, and as "X.YGHz"
/// if it is greater than or equal to 1000. If `speed_shorthand` is true, the
/// speed is rounded to the nearest tenth of a GHz before formatting.
///
/// The temperature is formatted as "[X.Y]°C" or "[X.Y]°F" depending on
/// `temp_unit`. If `temp_unit` is not specified, the temperature is in
/// Celsius.
pub fn get_cpu(
    cpu_brand: bool,
    show_freq: bool,
    show_cores: bool,
    show_temp: bool,
    speed_shorthand: bool,
    temp_unit: Option<char>,
) -> Option<String> {
    let cpuinfo = fs::read_to_string("/proc/cpuinfo").ok()?;
    let mut cpu_model = extract_cpu_model(&cpuinfo);
    let cores = if show_cores {
        Some(count_cores(&cpuinfo))
    } else {
        None
    };
    let speed = if show_freq {
        extract_speed(&cpuinfo)
    } else {
        None
    };
    let temp = if show_temp {
        extract_temp("/sys/class/hwmon/")
    } else {
        None
    };

    cpu_model = sanitize_cpu_model(&cpu_model, cpu_brand);
    let mut output = cpu_model;

    if let Some(c) = cores {
        output = format!("{} ({})", output, c);
    }

    if let Some(s) = speed {
        let formatted = if s < 1000 {
            format!("{}MHz", s)
        } else {
            let mut ghz = s as f32 / 1000.0;
            if speed_shorthand {
                ghz = (ghz * 10.0).round() / 10.0;
            }
            format!("{:.1}GHz", ghz)
        };
        output = format!("{} @ {}", output, formatted);
    }

    if let Some(mut celsius) = temp {
        if let Some('F') = temp_unit {
            celsius = celsius * 9.0 / 5.0 + 32.0;
        }
        output = format!("{} [{:.1}°{}]", output, celsius, temp_unit.unwrap_or('C'));
    }

    Some(output)
}

fn extract_cpu_model(cpuinfo: &str) -> String {
    for line in cpuinfo.lines() {
        if line.contains("model name") || line.contains("Hardware") || line.contains("Processor") {
            if let Some((_, val)) = line.split_once(':') {
                return val.trim().to_string();
            }
        }
    }
    "Unknown CPU".to_string()
}

fn count_cores(cpuinfo: &str) -> u32 {
    cpuinfo
        .lines()
        .filter(|l| l.starts_with("processor"))
        .count() as u32
}

fn extract_speed(cpuinfo: &str) -> Option<u32> {
    for line in cpuinfo.lines() {
        if line.contains("cpu MHz") {
            if let Some((_, val)) = line.split_once(':') {
                return val.trim().parse::<f32>().ok().map(|v| v.round() as u32);
            }
        }
    }
    None
}

fn extract_temp(hwmon_root: &str) -> Option<f32> {
    let root = Path::new(hwmon_root);
    if !root.exists() {
        return None;
    }

    for entry in fs::read_dir(root).ok()? {
        let path = entry.ok()?.path();
        let name_path = path.join("name");

        let name = fs::read_to_string(name_path).ok()?;
        if name.contains("coretemp") || name.contains("k10temp") || name.contains("cpu_thermal") {
            if let Ok(entries) = fs::read_dir(&path) {
                for entry in entries.flatten() {
                    let file_name = entry.file_name();
                    let file_name_str = file_name.to_string_lossy();
                    if file_name_str.starts_with("temp") && file_name_str.ends_with("_input") {
                        if let Ok(content) = fs::read_to_string(entry.path()) {
                            if let Ok(raw) = content.trim().parse::<f32>() {
                                return Some(raw / 1000.0);
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

fn sanitize_cpu_model(model: &str, show_brand: bool) -> String {
    let mut s = model.to_string();

    let replacements = [
        "(TM)",
        "(tm)",
        "(R)",
        "(r)",
        "CPU",
        "Processor",
        "Dual-Core",
        "Quad-Core",
        "Six-Core",
        "Eight-Core",
        "with Radeon",
        "FPU",
        "Technologies, Inc",
        "Core2",
        "Chip Revision",
        "Compute Cores",
        "Core ",
    ];

    for pat in replacements.iter() {
        s = s.replace(pat, "");
    }

    if !show_brand {
        let brands = ["AMD ", "Intel ", "Qualcomm ", "Core? Duo ", "Apple "];
        for brand in brands.iter() {
            s = s.replacen(brand, "", 1);
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs::{self, File};
    use std::io::Write;

    const MOCK_CPUINFO: &str = r#"
processor   : 0
vendor_id   : GenuineIntel
cpu MHz     : 2200.000
model name  : Intel(R) Core(TM) i7-8550U CPU @ 1.80GHz

processor   : 1
cpu MHz     : 2200.000
"#;

    const MOCK_CPUINFO_DECIMAL: &str = r#"
processor   : 0
cpu MHz     : 2199.6
"#;

    #[test]
    fn test_extract_cpu_model() {
        let model = extract_cpu_model(MOCK_CPUINFO);
        assert_eq!(model, "Intel(R) Core(TM) i7-8550U CPU @ 1.80GHz");
    }

    #[test]
    fn test_extract_cpu_model_unknown() {
        let empty_info = "";
        assert_eq!(extract_cpu_model(empty_info), "Unknown CPU");
    }

    #[test]
    fn test_count_cores() {
        let core_count = count_cores(MOCK_CPUINFO);
        assert_eq!(core_count, 2);
    }

    #[test]
    fn test_extract_speed() {
        let speed = extract_speed(MOCK_CPUINFO);
        assert_eq!(speed, Some(2200));
    }

    #[test]
    fn test_extract_speed_rounds_up() {
        let speed = extract_speed(MOCK_CPUINFO_DECIMAL);
        assert_eq!(speed, Some(2200));
    }

    #[test]
    fn test_sanitize_cpu_model_with_brand() {
        let input = "Intel(R) Core(TM) i7-8550U CPU @ 1.80GHz";
        let result = sanitize_cpu_model(input, true);
        assert!(result.contains("Intel"));
        assert!(result.contains("i7-8550U"));
        assert!(!result.contains("CPU"));
    }

    #[test]
    fn test_sanitize_cpu_model_strips_noise() {
        let input = "AMD Ryzen(TM) 5 5600X 6-Core Processor";
        let result = sanitize_cpu_model(input, true);
        assert!(!result.contains("(TM)"));
        assert!(!result.contains("6-Core"));
        assert!(result.contains("Ryzen"));
    }

    #[test]
    fn test_sanitize_cpu_model_without_brand() {
        let input = "Intel(R) Core(TM) i7-8550U CPU @ 1.80GHz";
        let result = sanitize_cpu_model(input, false);
        assert!(!result.contains("Intel"));
        assert!(result.contains("i7-8550U"));
    }

    #[test]
    fn test_extract_temp_with_mock_hwmon() {
        let temp_dir = env::temp_dir().join("test_hwmon");
        let hwmon_path = temp_dir.join("hwmon0");
        let name_path = hwmon_path.join("name");
        let temp_input_path = hwmon_path.join("temp1_input");

        fs::create_dir_all(&hwmon_path).unwrap();

        let mut name_file = File::create(&name_path).unwrap();
        writeln!(name_file, "coretemp").unwrap();

        let mut temp_file = File::create(&temp_input_path).unwrap();
        writeln!(temp_file, "47000").unwrap(); // 47.0°C

        let result = extract_temp(temp_dir.to_str().unwrap());
        assert_eq!(result, Some(47.0));

        fs::remove_dir_all(&temp_dir).unwrap();
    }

    #[test]
    fn test_get_cpu_basic() {
        // This test will only validate that the function returns something,
        // since it depends on system files.
        let result = get_cpu(true, true, true, false, false, None);
        assert!(result.is_some());
        let output = result.unwrap();
        assert!(!output.is_empty());
        println!("CPU Info: {}", output);
    }
}
