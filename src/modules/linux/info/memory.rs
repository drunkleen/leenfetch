use std::fs;

use crate::modules::enums::MemoryUnit;

pub fn get_memory(memory_percent: bool, memory_unit: MemoryUnit) -> Option<String> {
    let contents = fs::read_to_string("/proc/meminfo").ok()?;

    let mut mem_total_kb = 0;
    let mut mem_available_kb = None;
    let mut shmem = 0;
    let mut memfree = 0;
    let mut buffers = 0;
    let mut cached = 0;
    let mut sreclaimable = 0;

    for line in contents.lines() {
        let mut parts = line.split_whitespace();
        match parts.next()? {
            "MemTotal:" => mem_total_kb = parts.next()?.parse::<u64>().ok()?,
            "MemAvailable:" => mem_available_kb = parts.next()?.parse::<u64>().ok(),
            "Shmem:" => shmem = parts.next()?.parse::<u64>().ok()?,
            "MemFree:" => memfree = parts.next()?.parse::<u64>().ok()?,
            "Buffers:" => buffers = parts.next()?.parse::<u64>().ok()?,
            "Cached:" => cached = parts.next()?.parse::<u64>().ok()?,
            "SReclaimable:" => sreclaimable = parts.next()?.parse::<u64>().ok()?,
            _ => {}
        }
    }

    let used_kb = if let Some(avail) = mem_available_kb {
        mem_total_kb.saturating_sub(avail)
    } else {
        mem_total_kb + shmem - memfree - buffers - cached - sreclaimable
    };

    let (used, total, unit_str) = match memory_unit {
        MemoryUnit::GiB => (
            used_kb as f64 / 1024.0 / 1024.0,
            mem_total_kb as f64 / 1024.0 / 1024.0,
            "GiB",
        ),
        MemoryUnit::KiB => (used_kb as f64, mem_total_kb as f64, "KiB"),
        MemoryUnit::MiB => (used_kb as f64 / 1024.0, mem_total_kb as f64 / 1024.0, "MiB"),
    };

    let percent = if memory_percent {
        format!(" ({:.0}%)", used / total * 100.0)
    } else {
        "".to_string()
    };

    Some(format!(
        "{:.1}{} / {:.1}{}{}",
        used, unit_str, total, unit_str, percent
    ))
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
