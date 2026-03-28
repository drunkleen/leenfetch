use std::env;
use std::ffi::CString;
use std::fs;

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
    // 1. Try $USER environment variable (fastest)
    if let Some(u) = env::var_os("USER") {
        let s = u.to_string_lossy();
        if !s.is_empty() {
            return s.into();
        }
    }

    // 2. Use getuid() + getpwuid() syscalls (no process spawn)
    let uid = unsafe { libc::getuid() };
    if uid > 0 {
        // Try to get username from passwd entry
        if let Some(pw) = get_pwuid(uid) {
            return pw;
        }
    }

    // 3. Fallback: extract from $HOME
    if let Ok(home) = env::var("HOME") {
        if let Some(name) = home.rsplit('/').find(|s| !s.is_empty()) {
            return name.to_string();
        }
    }

    // 4. Worst-case
    "unknown".into()
}

fn get_pwuid(uid: libc::uid_t) -> Option<String> {
    // Use getpwuid_r for thread-safe passwd lookup
    let mut passwd = MaybeUninit::uninit();
    let mut buf = vec![0u8; 1024];

    let result = unsafe {
        let mut pwd_ptr: *mut libc::passwd = std::ptr::null_mut();
        libc::getpwuid_r(
            uid,
            passwd.as_mut_ptr(),
            buf.as_mut_ptr() as *mut libc::c_char,
            buf.len(),
            &mut pwd_ptr,
        )
    };

    if result == 0 && !passwd.as_ptr().is_null() {
        let pwd = unsafe { passwd.assume_init() };
        if !pwd.pw_name.is_null() {
            let name = unsafe { CString::from_raw(pwd.pw_name) };
            return Some(name.to_string_lossy().into_owned());
        }
    }
    None
}

fn get_hostname(fqdn: bool) -> String {
    // 1. Try gethostname() syscall first (fastest)
    let mut buf = [0u8; 256];
    let len = unsafe { libc::gethostname(buf.as_mut_ptr() as *mut libc::c_char, buf.len()) };
    if len == 0 {
        let s = String::from_utf8_lossy(&buf)
            .trim_end_matches('\0')
            .to_string();
        if !s.is_empty() && (s != "localhost" || !fqdn) {
            if fqdn {
                // Try to get FQDN
                if let Ok(fqdn_name) = fs::read_to_string("/etc/hostname") {
                    let trimmed = fqdn_name.trim().to_string();
                    if !trimmed.is_empty() && trimmed.contains('.') {
                        return trimmed;
                    }
                }
                // Try DNS domain from /etc/resolv.conf or nsswitch
                if let Ok(domain) = fs::read_to_string("/etc/resolv.conf") {
                    for line in domain.lines() {
                        if line.starts_with("domain ") {
                            let domain = line[7..].trim();
                            if !domain.is_empty() {
                                return format!("{}.{}", s, domain);
                            }
                        }
                    }
                }
            }
            return s;
        }
    }

    // 2. Try HOSTNAME environment variable
    if let Some(h) = env::var_os("HOSTNAME") {
        let s = h.to_string_lossy();
        if !s.is_empty() {
            return s.into();
        }
    }

    // 3. Fallback: read /etc/hostname
    if let Ok(hostname) = fs::read_to_string("/etc/hostname") {
        let s = hostname.trim().to_string();
        if !s.is_empty() {
            return s;
        }
    }

    "localhost".into()
}

use std::mem::MaybeUninit;

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
