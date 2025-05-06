use libc::{uname, utsname};
use std::ffi::CStr;

#[inline(always)]
pub fn get_kernel() -> Option<String> {
    unsafe {
        let mut uts: utsname = std::mem::zeroed();
        if uname(&mut uts) != 0 {
            return None;
        }

        let release = CStr::from_ptr(uts.release.as_ptr())
            .to_str()
            .ok()?
            .to_owned(); // allocates a proper String

        Some(release)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kernel_version_is_some() {
        let version = get_kernel();
        assert!(version.is_some(), "Kernel version should not be None");
    }

    #[test]
    fn kernel_version_is_not_empty() {
        let version = get_kernel().expect("Expected Some version string");
        assert!(
            !version.trim().is_empty(),
            "Kernel version should not be empty"
        );
    }

    #[test]
    fn kernel_version_format_valid() {
        let version = get_kernel().expect("Expected Some version string");

        // Simple check: must contain at least one dot (e.g., "6.5.0")
        assert!(
            version.contains('.'),
            "Kernel version format is suspicious: {}",
            version
        );
    }
}
