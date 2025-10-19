use std::ptr::null_mut;
use winapi::shared::minwindef::DWORD;
use winapi::shared::winerror::ERROR_SUCCESS;
use winapi::um::winnt::OSVERSIONINFOW;
use winapi::um::winreg::{HKEY_LOCAL_MACHINE, RRF_RT_REG_DWORD, RegGetValueW};

#[inline(always)]
pub fn get_kernel() -> Option<String> {
    // Prefer RtlGetVersion for accurate kernel version
    unsafe {
        let mut vi: OSVERSIONINFOW = std::mem::zeroed();
        vi.dwOSVersionInfoSize = std::mem::size_of::<OSVERSIONINFOW>() as u32;
        let status = RtlGetVersion(&mut vi as *mut _);
        if status == 0 {
            let mut s = format!(
                "{}.{}.{}",
                vi.dwMajorVersion, vi.dwMinorVersion, vi.dwBuildNumber
            );
            // Append UBR (revision) if present in registry
            if let Some(ubr) = read_ubr() {
                s.push('.');
                s.push_str(&ubr.to_string());
            }
            return Some(s);
        }
    }
    None
}

fn read_ubr() -> Option<DWORD> {
    let key = to_wide("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion");
    let val = to_wide("UBR");
    let mut data: DWORD = 0;
    let mut size = std::mem::size_of::<DWORD>() as u32;
    let status = unsafe {
        RegGetValueW(
            HKEY_LOCAL_MACHINE,
            key.as_ptr(),
            val.as_ptr(),
            RRF_RT_REG_DWORD,
            null_mut(),
            &mut data as *mut _ as *mut _,
            &mut size,
        )
    };
    if status == ERROR_SUCCESS as i32 {
        Some(data)
    } else {
        None
    }
}

fn to_wide(s: &str) -> Vec<u16> {
    use std::os::windows::ffi::OsStrExt;
    std::ffi::OsStr::new(s)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

#[link(name = "ntdll")]
unsafe extern "system" {
    fn RtlGetVersion(lpVersionInformation: *mut OSVERSIONINFOW) -> i32;
}
