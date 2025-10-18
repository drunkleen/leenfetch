use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn get_gpus() -> Vec<String> {
    let mut entries = collect_from_sysfs();
    if entries.is_empty() {
        entries = collect_from_lspci();
    }

    if entries.is_empty() {
        return vec!["Unknown GPU".to_string()];
    }

    entries
        .into_iter()
        .enumerate()
        .map(|(idx, info)| format!("{idx}: {info}"))
        .collect()
}

fn collect_from_sysfs() -> Vec<String> {
    collect_from_sysfs_root(Path::new("/sys/class/drm"))
}

fn collect_from_sysfs_root(root: &Path) -> Vec<String> {
    let Ok(read_dir) = fs::read_dir(root) else {
        return Vec::new();
    };
    let mut out = Vec::new();

    for entry in read_dir.flatten() {
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if !name.starts_with("card") || name.contains('-') {
            continue;
        }

        let device_dir = entry.path().join("device");
        if !device_dir.is_dir() {
            continue;
        }

        if let Some(line) = describe_device(&device_dir) {
            out.push(line);
        }
    }

    out
}

fn describe_device(device_dir: &Path) -> Option<String> {
    let vendor_hex = read_trimmed(device_dir.join("vendor")).and_then(|s| normalize_hex(&s));
    let device_hex = read_trimmed(device_dir.join("device")).and_then(|s| normalize_hex(&s));
    let driver_path = device_dir.join("driver");
    let driver = fs::read_link(&driver_path)
        .ok()
        .and_then(|path| path.file_name().map(|s| s.to_string_lossy().into_owned()))
        .or_else(|| {
            read_trimmed(&driver_path).and_then(|raw| {
                Path::new(&raw)
                    .file_name()
                    .map(|s| s.to_string_lossy().into_owned())
            })
        });

    let vendor_id = vendor_hex
        .as_deref()
        .and_then(|hex| u16::from_str_radix(hex, 16).ok());
    let device_id = device_hex
        .as_deref()
        .and_then(|hex| u16::from_str_radix(hex, 16).ok());

    let db = pci_database();
    let vendor_name = vendor_id.and_then(|id| {
        db.as_ref()
            .and_then(|db| db.vendors.get(&id).cloned())
            .map(|s| s.trim().to_string())
    });
    let device_name = if let (Some(vendor_id), Some(device_id)) = (vendor_id, device_id) {
        db.as_ref()
            .and_then(|db| db.devices.get(&(vendor_id, device_id)).cloned())
            .map(|s| s.trim().to_string())
    } else {
        None
    };

    let mut label = match (vendor_name, device_name) {
        (Some(vendor), Some(model)) => format!("{vendor} {model}"),
        (Some(vendor), None) => vendor,
        (None, Some(model)) => model,
        _ => format!(
            "GPU [{}:{}]",
            vendor_hex.as_deref().unwrap_or("????"),
            device_hex.as_deref().unwrap_or("????")
        ),
    };

    if let Some(role) = classify_gpu(vendor_id, driver.as_deref()) {
        label.push_str(&format!(" [{}]", role));
    }

    Some(label)
}

fn collect_from_lspci() -> Vec<String> {
    let output = Command::new("lspci")
        .arg("-mm")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_default();

    let mut gpus = Vec::new();

    for line in output.lines() {
        if !(line.contains("\"VGA") || line.contains("\"3D") || line.contains("\"Display")) {
            continue;
        }

        let parts: Vec<&str> = line.split('"').collect();
        if parts.len() < 6 {
            continue;
        }

        let vendor = parts[3].trim();
        let model = parts[5].trim();
        if vendor.is_empty() && model.is_empty() {
            continue;
        }

        let mut label = format!("{vendor} {model}").trim().to_string();
        let role = classify_gpu_from_name(&label);
        if let Some(role) = role {
            label.push_str(&format!(" [{}]", role));
        }

        gpus.push(label);
    }

    gpus
}

fn read_trimmed<P: AsRef<Path>>(path: P) -> Option<String> {
    let contents = fs::read_to_string(path).ok()?;
    let trimmed = contents.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn normalize_hex(value: &str) -> Option<String> {
    let trimmed = value
        .trim()
        .trim_start_matches("0x")
        .trim_start_matches("0X");
    if trimmed.is_empty() {
        return None;
    }
    u16::from_str_radix(trimmed, 16)
        .ok()
        .map(|v| format!("{:04X}", v))
}

fn classify_gpu(vendor: Option<u16>, driver: Option<&str>) -> Option<&'static str> {
    match vendor {
        Some(0x8086) => Some("Integrated"),
        Some(0x1002) | Some(0x1022) | Some(0x10DE) => Some("Discrete"),
        Some(0x1234) | Some(0x1AF4) | Some(0x1B36) => Some("Virtual"),
        Some(0x1A03) => Some("BMC"),
        _ => match driver {
            Some("i915") | Some("xe") => Some("Integrated"),
            Some("amdgpu") | Some("radeon") | Some("nvidia") => Some("Discrete"),
            Some("virtio-pci") | Some("bochs-drm") => Some("Virtual"),
            _ => None,
        },
    }
}

fn classify_gpu_from_name(name: &str) -> Option<&'static str> {
    let lower = name.to_ascii_lowercase();
    if lower.contains("intel") {
        Some("Integrated")
    } else if lower.contains("nvidia") || lower.contains("geforce") || lower.contains("radeon") {
        Some("Discrete")
    } else if lower.contains("virtio") || lower.contains("qxl") || lower.contains("vmware") {
        Some("Virtual")
    } else {
        None
    }
}

struct PciDatabase {
    vendors: HashMap<u16, String>,
    devices: HashMap<(u16, u16), String>,
}

static PCI_DB: Lazy<Option<PciDatabase>> = Lazy::new(load_pci_database);

fn pci_database() -> &'static Option<PciDatabase> {
    &*PCI_DB
}

fn load_pci_database() -> Option<PciDatabase> {
    if let Ok(custom) = std::env::var("LEENFETCH_PCI_IDS") {
        if let Ok(contents) = fs::read_to_string(&custom) {
            return Some(parse_pci_ids(&contents));
        }
    }

    for candidate in ["/usr/share/hwdata/pci.ids", "/usr/share/misc/pci.ids"] {
        if let Ok(contents) = fs::read_to_string(candidate) {
            return Some(parse_pci_ids(&contents));
        }
    }

    None
}

fn parse_pci_ids(contents: &str) -> PciDatabase {
    let mut vendors = HashMap::new();
    let mut devices = HashMap::new();
    let mut current_vendor = None;

    for line in contents.lines() {
        let line = line.trim_end();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some(rest) = line.strip_prefix('\t') {
            if rest.starts_with('\t') {
                continue;
            }
            if let (Some(vendor_id), Some(device_id)) = (current_vendor, parse_id(rest)) {
                devices.insert((vendor_id, device_id), rest[4..].trim().to_string());
            }
        } else if let Some(vendor_id) = parse_id(line) {
            vendors.insert(vendor_id, line[4..].trim().to_string());
            current_vendor = Some(vendor_id);
        }
    }

    PciDatabase { vendors, devices }
}

fn parse_id(line: &str) -> Option<u16> {
    if line.len() < 4 {
        return None;
    }
    u16::from_str_radix(&line[0..4], 16).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn test_normalize_hex() {
        assert_eq!(normalize_hex("0x10de"), Some("10DE".into()));
        assert_eq!(normalize_hex("10DE"), Some("10DE".into()));
        assert_eq!(normalize_hex(""), None);
    }

    #[test]
    fn test_classify_from_name() {
        assert_eq!(
            classify_gpu_from_name("Intel UHD Graphics"),
            Some("Integrated")
        );
        assert_eq!(
            classify_gpu_from_name("NVIDIA GeForce RTX 3050 Mobile"),
            Some("Discrete")
        );
        assert_eq!(classify_gpu_from_name("QXL GPU"), Some("Virtual"));
        assert_eq!(classify_gpu_from_name("Mystery Adapter"), None);
    }

    #[test]
    fn test_collect_from_sysfs_formatting() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let temp = std::env::temp_dir().join(format!("leenfetch_gpu_test_{unique}"));
        fs::create_dir_all(temp.join("card0/device")).unwrap();
        fs::write(temp.join("card0/device/vendor"), "0x8086\n").unwrap();
        fs::write(temp.join("card0/device/device"), "0x9A60\n").unwrap();
        fs::write(temp.join("card0/device/driver"), "i915\n").unwrap();

        let database = "\
8086  Intel Corporation
\t9A60  Alder Lake-P GT1 [UHD Graphics]
";
        let db_path = temp.join("pci.ids");
        fs::write(&db_path, database).unwrap();
        env::set_var("LEENFETCH_PCI_IDS", db_path.to_str().unwrap());

        let result = super::collect_from_sysfs_root(temp.as_path());
        assert_eq!(
            result,
            vec!["Intel Corporation Alder Lake-P GT1 [UHD Graphics] [Integrated]"]
        );

        env::remove_var("LEENFETCH_PCI_IDS");
        fs::remove_dir_all(temp).unwrap();
    }
}
