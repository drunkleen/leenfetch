use std::env;

pub fn get_titles(fqdn: bool) -> (String, String, usize) {
    let user = get_user();
    let host = get_hostname(fqdn);

    let len = user.len() + host.len() + 1; // include '@' or separator
    (user, host, len)
}

fn get_user() -> String {
    if let Some(u) = env::var_os("USERNAME") {
        let s = u.to_string_lossy();
        if !s.is_empty() {
            return s.into();
        }
    }

    "unknown".into()
}

fn get_hostname(fqdn: bool) -> String {
    if fqdn {
        // Try USERDNSDOMAIN or USERDOMAIN + COMPUTERNAME
        if let Some(dns) = env::var_os("USERDNSDOMAIN") {
            let domain = dns.to_string_lossy();
            if !domain.is_empty() {
                if let Some(name) = env::var_os("COMPUTERNAME") {
                    let host = name.to_string_lossy();
                    if !host.is_empty() {
                        return format!("{}.{}", host, domain).to_lowercase();
                    }
                }
            }
        }
    }

    if let Some(name) = env::var_os("COMPUTERNAME") {
        let host = name.to_string_lossy();
        if !host.is_empty() {
            return host.into();
        }
    }

    "localhost".into()
}
