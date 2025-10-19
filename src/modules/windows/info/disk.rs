use crate::modules::{
    enums::{DiskDisplay, DiskSubtitle},
    utils::get_bar,
};
use std::ffi::OsString;
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use winapi::shared::minwindef::DWORD;
use winapi::um::fileapi::{GetDiskFreeSpaceExW, GetDriveTypeW, GetLogicalDriveStringsW};
use winapi::um::winbase::{DRIVE_FIXED};

pub fn get_disks(
    subtitle_mode: DiskSubtitle,
    display_mode: DiskDisplay,
    _paths: Option<Vec<&str>>,
) -> Option<Vec<(String, String)>> {
    let drives = enumerate_fixed_drives();
    let mut results = Vec::new();

    for root in drives {
        if let Some((total, free)) = query_space(&root) {
            if total == 0 { continue; }
            let used = total.saturating_sub(free);
            let perc = ((used as f64 / total as f64) * 100.0).round() as u8;
            let used_str = format!("{:.1}G", used as f64 / 1024.0 / 1024.0 / 1024.0);
            let total_str = format!("{:.1}G", total as f64 / 1024.0 / 1024.0 / 1024.0);
            let usage_display = format!("{} / {}", used_str, total_str);
            let bar = get_bar(perc);
            let final_str = match display_mode {
                DiskDisplay::Info => usage_display.clone(),
                DiskDisplay::Percentage => format!("{}% {}", perc, bar),
                DiskDisplay::InfoBar => format!("{} {}", usage_display, bar),
                DiskDisplay::BarInfo => format!("{} {}", bar, usage_display),
                DiskDisplay::Bar => bar,
            };
            let mount = root.trim_end_matches('\u{0}').trim_end_matches('\\').to_string();
            let subtitle = match subtitle_mode {
                DiskSubtitle::Name => mount.clone(),
                DiskSubtitle::Dir => mount.trim_end_matches(':').to_string(),
                DiskSubtitle::Mount => mount.clone(),
                DiskSubtitle::None => String::new(),
            };
            let label = if subtitle.is_empty() { "Disk".to_string() } else { format!("Disk ({})", subtitle) };
            results.push((label, final_str));
        }
    }

    Some(results)
}

fn enumerate_fixed_drives() -> Vec<String> {
    unsafe {
        // Get required buffer length
        let mut buffer: [u16; 512] = [0; 512];
        let len = GetLogicalDriveStringsW(buffer.len() as DWORD, buffer.as_mut_ptr());
        if len == 0 || (len as usize) >= buffer.len() {
            return Vec::new();
        }
        let mut drives = Vec::new();
        let mut start = 0usize;
        for i in 0..(len as usize) {
            if buffer[i] == 0 {
                if i > start {
                    let slice = &buffer[start..i];
                    let s = OsString::from_wide(slice).to_string_lossy().to_string();
                    // Filter to fixed drives
                    let w: Vec<u16> = slice.iter().copied().chain(std::iter::once(0)).collect();
                    if GetDriveTypeW(w.as_ptr()) == DRIVE_FIXED {
                        drives.push(s);
                    }
                }
                start = i + 1;
            }
        }
        drives
    }
}

fn query_space(root: &str) -> Option<(u64, u64)> {
    unsafe {
        let mut free_avail: u64 = 0;
        let mut total: u64 = 0;
        let mut free_total: u64 = 0;
        let path_w: Vec<u16> = OsString::from(root).encode_wide().chain(std::iter::once(0)).collect();
        let ok = GetDiskFreeSpaceExW(
            path_w.as_ptr(),
            &mut free_avail as *mut _ as *mut _,
            &mut total as *mut _ as *mut _,
            &mut free_total as *mut _ as *mut _,
        );
        if ok == 0 {
            return None;
        }
        Some((total, free_total))
    }
}
