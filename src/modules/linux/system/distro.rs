use std::fs;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DistroDisplay {
    Name,
    NameVersion,
    NameArch,
    NameModel,
    NameModelVersion,
    NameModelArch,
    NameModelVersionArch,
}

impl FromStr for DistroDisplay {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        match s.to_lowercase().as_str() {
            "name" => Ok(Self::Name),
            "name_version" => Ok(Self::NameVersion),
            "name_arch" => Ok(Self::NameArch),
            "name_model" => Ok(Self::NameModel),
            "name_model_version" => Ok(Self::NameModelVersion),
            "name_model_arch" => Ok(Self::NameModelArch),
            "name_model_version_arch" => Ok(Self::NameModelVersionArch),
            _ => Err(()),
        }
    }
}

pub fn get_distro(format: DistroDisplay) -> String {
    let release_files = [
        "/etc/os-release",
        "/usr/lib/os-release",
        "/etc/lsb-release",
        "/etc/openwrt_release",
    ];

    for path in release_files {
        if Path::new(path).exists() {
            if let Ok(contents) = fs::read_to_string(path) {
                return parse_distro_info(&contents, format);
            }
        }
    }

    "Unknown".into()
}

fn parse_distro_info(contents: &str, format: DistroDisplay) -> String {
    let mut name = None;
    let mut version = None;
    let mut pretty = None;
    let mut description = None;
    let mut codename = None;

    for line in contents.lines() {
        let line = line.trim();
        if line.starts_with("NAME=") {
            name = Some(trim_quotes(&line[5..]));
        } else if line.starts_with("VERSION_ID=") {
            version = Some(trim_quotes(&line[11..]));
        } else if line.starts_with("PRETTY_NAME=") {
            pretty = Some(trim_quotes(&line[12..]));
        } else if line.starts_with("DISTRIB_DESCRIPTION=") {
            description = Some(trim_quotes(&line[21..]));
        } else if line.starts_with("VERSION_CODENAME=") {
            codename = Some(trim_quotes(&line[17..]));
        } else if line.starts_with("UBUNTU_CODENAME=") {
            codename = Some(trim_quotes(&line[17..]));
        } else if line.starts_with("TAILS_PRODUCT_NAME=") {
            name = Some(trim_quotes(&line[20..]));
        }
    }

    let name = name
        .or_else(|| pretty.clone())
        .or_else(|| description.clone())
        .unwrap_or_else(|| "Unknown".to_string());

    let version = version.or_else(|| {
        description.as_ref().and_then(|desc| {
            desc.split_whitespace()
                .find(|s| {
                    s.chars()
                        .next()
                        .map(|c| c.is_ascii_digit())
                        .unwrap_or(false)
                })
                .map(|s| s.to_string())
        })
    });

    let arch = std::env::consts::ARCH;
    let model = infer_model(&name, &codename, &description);

    match format {
        DistroDisplay::Name => name,
        DistroDisplay::NameVersion => format!("{name} {}", version.unwrap_or_default())
            .trim()
            .to_string(),
        DistroDisplay::NameArch => format!("{name} {}", arch).to_string(),
        DistroDisplay::NameModel => format!("{name} {}", model).trim().to_string(),
        DistroDisplay::NameModelVersion => {
            format!("{name} {} {}", model, version.unwrap_or_default())
                .trim()
                .to_string()
        }
        DistroDisplay::NameModelArch => format!("{name} {} {}", model, arch).trim().to_string(),
        DistroDisplay::NameModelVersionArch => {
            format!("{name} {} {} {}", model, version.unwrap_or_default(), arch)
                .trim()
                .to_string()
        }
    }
}

fn trim_quotes(s: &str) -> String {
    s.trim_matches('"').to_string()
}

struct DistroModel {
    keyword: &'static str,
    model: &'static str,
}

static MODEL_HINTS: &[DistroModel] = &[
    DistroModel {
        keyword: "arch",
        model: "Rolling",
    },
    DistroModel {
        keyword: "artix",
        model: "Rolling",
    },
    DistroModel {
        keyword: "endeavouros",
        model: "Rolling",
    },
    DistroModel {
        keyword: "manjaro",
        model: "Rolling",
    },
    DistroModel {
        keyword: "void",
        model: "Rolling",
    },
    DistroModel {
        keyword: "nixos",
        model: "Rolling",
    },
    DistroModel {
        keyword: "tumbleweed",
        model: "Rolling",
    },
    DistroModel {
        keyword: "rawhide",
        model: "Testing",
    },
    DistroModel {
        keyword: "testing",
        model: "Testing",
    },
    DistroModel {
        keyword: "stable",
        model: "Stable",
    },
    DistroModel {
        keyword: "ubuntu",
        model: "LTS",
    }, // fallback for known Ubuntu LTS
    DistroModel {
        keyword: "lts",
        model: "LTS",
    },
    DistroModel {
        keyword: "tails",
        model: "Stable",
    },
    DistroModel {
        keyword: "alpine",
        model: "Stable",
    },
    DistroModel {
        keyword: "debian",
        model: "Stable",
    }, // default to Stable unless Testing is seen
];

fn infer_model(name: &str, codename: &Option<String>, description: &Option<String>) -> String {
    let text = format!(
        "{} {} {}",
        name.to_lowercase(),
        codename.as_deref().unwrap_or("").to_lowercase(),
        description.as_deref().unwrap_or("").to_lowercase()
    );

    for entry in MODEL_HINTS {
        if text.contains(entry.keyword) {
            return entry.model.to_string();
        }
    }

    "Unknown".into()
}
