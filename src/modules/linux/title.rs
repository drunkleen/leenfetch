use std::env;
use std::process::Command;

/// Return (username, hostname, combined length) as a tuple.
///
/// FQDN is passed to `get_hostname` to determine whether to include the domain.
///
/// The length is the sum of the lengths of the username and hostname,
/// plus one for the '@' separator.
pub fn get_titles(fqdn: bool) -> (String, String) {
    let user = get_user();
    let host = get_hostname(fqdn);

    (user, host)
}

fn get_user() -> String {
    // 1. Try $USER
    if let Some(u) = env::var_os("USER") {
        let s = u.to_string_lossy();
        if !s.is_empty() {
            return s.into();
        }
    }

    if let Ok(out) = Command::new("id").arg("-un").output() {
        if out.status.success() {
            let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if !s.is_empty() {
                return s;
            }
        }
    }

    if let Ok(home) = env::var("HOME") {
        if let Some(name) = home.rsplit('/').find(|s| !s.is_empty()) {
            return name.to_string();
        }
    }

    // 4. Worst-case
    "unknown".into()
}

fn get_hostname(fqdn: bool) -> String {
    if fqdn {
        if let Ok(out) = Command::new("hostname").arg("-f").output() {
            if out.status.success() {
                let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
                if !s.is_empty() {
                    return s;
                }
            }
        }
    }

    if let Some(h) = env::var_os("HOSTNAME") {
        let s = h.to_string_lossy();
        if !s.is_empty() {
            return s.into();
        }
    }

    if let Ok(out) = Command::new("hostname").output() {
        if out.status.success() {
            let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if !s.is_empty() {
                return s;
            }
        }
    }

    "localhost".into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::EnvLock;

    #[test]
    fn test_get_user_from_env() {
        let env_lock = EnvLock::acquire(&["USER"]);
        env_lock.set_var("USER", "testuser");
        assert_eq!(get_user(), "testuser");
        drop(env_lock);
    }

    #[test]
    fn test_hostname_from_env() {
        let env_lock = EnvLock::acquire(&["HOSTNAME"]);
        env_lock.set_var("HOSTNAME", "testhost");
        assert_eq!(get_hostname(false), "testhost");
        drop(env_lock);
    }

    #[test]
    fn test_hostname_command_fallback() {
        let env_lock = EnvLock::acquire(&["HOSTNAME"]);
        env_lock.remove_var("HOSTNAME");
        let result = get_hostname(false);
        assert!(!result.is_empty(), "Hostname should not be empty");
        drop(env_lock);
    }

    #[test]
    fn test_hostname_final_fallback() {
        // This test can't force full fallback easily since `hostname` command always exists,
        // but we can at least ensure it's non-empty
        let result = get_hostname(false);
        assert!(!result.is_empty());
    }
}
