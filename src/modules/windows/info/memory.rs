use std::mem::{size_of, zeroed};
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum MemoryUnit {
    MiB,
    GiB,
    KiB,
}

impl FromStr for MemoryUnit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        match s.to_lowercase().as_str() {
            "mib" => Ok(MemoryUnit::MiB),
            "gib" => Ok(MemoryUnit::GiB),
            "kib" => Ok(MemoryUnit::KiB),
            _ => Err(()),
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
struct MEMORYSTATUSEX {
    dwLength: u32,
    dwMemoryLoad: u32,
    ullTotalPhys: u64,
    ullAvailPhys: u64,
    ullTotalPageFile: u64,
    ullAvailPageFile: u64,
    ullTotalVirtual: u64,
    ullAvailVirtual: u64,
    ullAvailExtendedVirtual: u64,
}

extern "system" {
    fn GlobalMemoryStatusEx(lpBuffer: *mut MEMORYSTATUSEX) -> i32;
}

pub fn get_memory(memory_percent: bool, memory_unit: MemoryUnit) -> Option<String> {
    unsafe {
        let mut mem_info: MEMORYSTATUSEX = zeroed();
        mem_info.dwLength = size_of::<MEMORYSTATUSEX>() as u32;

        if GlobalMemoryStatusEx(&mut mem_info as *mut _) == 0 {
            return None;
        }

        let total = mem_info.ullTotalPhys;
        let avail = mem_info.ullAvailPhys;
        let used = total.saturating_sub(avail);

        let (used_conv, total_conv, unit_str) = match memory_unit {
            MemoryUnit::GiB => (
                used as f64 / 1024.0 / 1024.0 / 1024.0,
                total as f64 / 1024.0 / 1024.0 / 1024.0,
                "GiB",
            ),
            MemoryUnit::MiB => (
                used as f64 / 1024.0 / 1024.0,
                total as f64 / 1024.0 / 1024.0,
                "MiB",
            ),
            MemoryUnit::KiB => (used as f64 / 1024.0, total as f64 / 1024.0, "KiB"),
        };

        let percent = if memory_percent {
            format!(" ({:.0}%)", used_conv / total_conv * 100.0)
        } else {
            "".to_string()
        };

        Some(format!(
            "{:.1}{} / {:.1}{}{}",
            used_conv, unit_str, total_conv, unit_str, percent
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_output_mib() {
        let result = get_memory(false, MemoryUnit::MiB);
        assert!(result.is_some());
        assert!(result.unwrap().contains("MiB"));
    }

    #[test]
    fn test_memory_output_gib_with_percent() {
        let result = get_memory(true, MemoryUnit::GiB);
        let output = result.unwrap();
        assert!(output.contains("GiB"));
        assert!(output.contains('%'));
    }

    #[test]
    fn test_memory_output_kib() {
        let result = get_memory(false, MemoryUnit::KiB);
        assert!(result.unwrap().contains("KiB"));
    }
}
