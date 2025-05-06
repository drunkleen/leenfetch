use std::fs;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum DistroShorthand {
    On,
    Tiny,
    Off,
}

impl FromStr for DistroShorthand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "on" => Ok(DistroShorthand::On),
            "tiny" => Ok(DistroShorthand::Tiny),
            "off" => Ok(DistroShorthand::Off),
            _ => Err(()),
        }
    }
}
pub fn get_distro(shorthand: DistroShorthand) -> Option<String> {
    let release_files = [
        "/etc/lsb-release",
        "/usr/lib/os-release",
        "/etc/os-release",
        "/etc/openwrt_release",
    ];

    for file in &release_files {
        if Path::new(file).exists() {
            if let Ok(contents) = fs::read_to_string(file) {
                return parse_distro_info(&contents, shorthand);
            }
        }
    }

    None
}

fn parse_distro_info(contents: &str, shorthand: DistroShorthand) -> Option<String> {
    let mut name = None;
    let mut version_id = None;
    let mut pretty_name = None;
    let mut distrib_id = None;
    let mut distrib_release = None;
    let mut distrib_description = None;
    let mut ubuntu_codename = None;
    let mut tails_product_name = None;

    for line in contents.lines() {
        if line.starts_with("NAME=") {
            name = Some(trim_quotes(line.trim_start_matches("NAME=")));
        } else if line.starts_with("VERSION_ID=") {
            version_id = Some(trim_quotes(line.trim_start_matches("VERSION_ID=")));
        } else if line.starts_with("PRETTY_NAME=") {
            pretty_name = Some(trim_quotes(line.trim_start_matches("PRETTY_NAME=")));
        } else if line.starts_with("DISTRIB_ID=") {
            distrib_id = Some(trim_quotes(line.trim_start_matches("DISTRIB_ID=")));
        } else if line.starts_with("DISTRIB_RELEASE=") {
            distrib_release = Some(trim_quotes(line.trim_start_matches("DISTRIB_RELEASE=")));
        } else if line.starts_with("DISTRIB_DESCRIPTION=") {
            distrib_description =
                Some(trim_quotes(line.trim_start_matches("DISTRIB_DESCRIPTION=")));
        } else if line.starts_with("UBUNTU_CODENAME=") {
            ubuntu_codename = Some(trim_quotes(line.trim_start_matches("UBUNTU_CODENAME=")));
        } else if line.starts_with("TAILS_PRODUCT_NAME=") {
            tails_product_name = Some(trim_quotes(line.trim_start_matches("TAILS_PRODUCT_NAME=")));
        }
    }

    let distro = match shorthand {
        DistroShorthand::On => {
            let name = name.or(distrib_id).unwrap_or_else(|| "Unknown".to_string());
            let version = version_id
                .or(distrib_release)
                .unwrap_or_else(|| "".to_string());
            format!("{} {}", name, version).trim().to_string()
        }
        DistroShorthand::Tiny => name
            .or(distrib_id)
            .or(tails_product_name)
            .unwrap_or_else(|| "Unknown".to_string()),
        DistroShorthand::Off => {
            let name = pretty_name
                .or(distrib_description)
                .unwrap_or_else(|| "Unknown".to_string());
            let suffix = ubuntu_codename.unwrap_or_else(|| "".to_string());
            format!("{} {}", name, suffix).trim().to_string()
        }
    };

    Some(distro)
}

fn trim_quotes(s: &str) -> String {
    s.trim_matches('"').to_string()
}

#[cfg(test)]
mod tests {
    use super::DistroShorthand;
    use super::*;

    #[test]
    fn test_parse_distro_info_on() {
        let contents = r#"
NAME="Arch Linux"
VERSION_ID="2023.10.01"
"#;
        let result = parse_distro_info(contents, DistroShorthand::On);
        assert_eq!(result.unwrap(), "Arch Linux 2023.10.01");
    }

    #[test]
    fn test_parse_distro_info_tiny() {
        let contents = r#"
NAME="Fedora"
"#;
        let result = parse_distro_info(contents, DistroShorthand::Tiny);
        assert_eq!(result.unwrap(), "Fedora");
    }

    #[test]
    fn test_parse_distro_info_off_with_codename() {
        let contents = r#"
PRETTY_NAME="Ubuntu 22.04 LTS"
UBUNTU_CODENAME=jammy
"#;
        let result = parse_distro_info(contents, DistroShorthand::Off);
        assert_eq!(result.unwrap(), "Ubuntu 22.04 LTS jammy");
    }

    #[test]
    fn test_parse_distro_info_fallback_to_distrib_fields() {
        let contents = r#"
DISTRIB_ID="Debian"
DISTRIB_RELEASE="12"
DISTRIB_DESCRIPTION="Debian GNU/Linux 12 (bookworm)"
"#;
        let result_on = parse_distro_info(contents, DistroShorthand::On);
        assert_eq!(result_on.unwrap(), "Debian 12");

        let result_off = parse_distro_info(contents, DistroShorthand::Off);
        assert_eq!(result_off.unwrap(), "Debian GNU/Linux 12 (bookworm)");
    }

    #[test]
    fn test_parse_distro_info_unknown() {
        let contents = r#"
SOME_UNKNOWN_FIELD=xyz
"#;
        let result = parse_distro_info(contents, DistroShorthand::On);
        assert_eq!(result.unwrap(), "Unknown");
    }

    #[test]
    fn test_trim_quotes_works() {
        assert_eq!(trim_quotes("\"quoted\""), "quoted");
        assert_eq!(trim_quotes("no_quotes"), "no_quotes");
    }
}
