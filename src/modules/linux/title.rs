use std::env;
use std::process::Command;

/// Return the username and hostname (with fqdn if `fqdn` is true), and their combined length.
/// fqdn [fully qualified hostname]
///
/// If the username or hostname cannot be determined, it is replaced with "unknown" or "localhost"
/// respectively.
pub fn get_titles(fqdn: bool) -> (String, String, usize) {
    let user = get_user().unwrap_or_else(|| "unknown".to_string());
    let hostname = get_hostname(fqdn).unwrap_or_else(|| "localhost".to_string());

    let length = user.len() + hostname.len() + 1;

    (user, hostname, length)
}

fn get_user() -> Option<String> {
    // Try $USER
    if let Ok(user) = env::var("USER") {
        if !user.is_empty() {
            return Some(user);
        }
    }

    // Fallback: `id -un`
    if let Ok(output) = Command::new("id").arg("-un").output() {
        if output.status.success() {
            let out = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !out.is_empty() {
                return Some(out);
            }
        }
    }

    // Fallback: use last component of $HOME
    if let Ok(home) = env::var("HOME") {
        if let Some(user) = home.rsplit('/').next() {
            if !user.is_empty() {
                return Some(user.to_string());
            }
        }
    }

    None
}

fn get_hostname(fqdn: bool) -> Option<String> {
    if fqdn {
        if let Ok(output) = Command::new("hostname").arg("-f").output() {
            if output.status.success() {
                return Some(String::from_utf8_lossy(&output.stdout).trim().to_string());
            }
        }
    } else {
        if let Ok(hostname) = env::var("HOSTNAME") {
            if !hostname.is_empty() {
                return Some(hostname);
            }
        }

        if let Ok(output) = Command::new("hostname").output() {
            if output.status.success() {
                return Some(String::from_utf8_lossy(&output.stdout).trim().to_string());
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_title_fqdn_off() {
        let (user, hostname, len) = get_titles(false);
        assert_eq!(user.len() + hostname.len() + 1, len);
    }

    #[test]
    fn test_get_title_fqdn_on() {
        let (user, hostname, len) = get_titles(true);
        assert_eq!(user.len() + hostname.len() + 1, len);
    }

    #[test]
    fn test_user_fallback() {
        let user = get_user();
        assert!(user.is_some());
    }

    #[test]
    fn test_hostname_fallback() {
        let name = get_hostname(false);
        assert!(name.is_some());
    }
}
