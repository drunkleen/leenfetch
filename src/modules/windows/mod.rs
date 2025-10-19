pub mod desktop;
pub mod info;
pub mod packages;
pub mod shell;
pub mod song;
pub mod system;
pub mod title;

// Windows-only universal smoke tests for the Windows modules.
// These tests favor robustness in diverse environments (CI, VMs, headless),
// asserting non-panics and minimal expectations rather than strict values.
#[cfg(all(test, windows))]
mod tests {
    use super::*;
    use crate::modules::enums::{
        BatteryDisplayMode, DistroDisplay, DiskDisplay, DiskSubtitle, MemoryUnit, OsAgeShorthand,
        PackageShorthand, UptimeShorthand,
    };

    #[test]
    fn kernel_version_reports_something() {
        let ver = system::kernel::get_kernel();
        assert!(ver.is_some(), "kernel version should be detectable on Windows");
        assert!(!ver.unwrap().trim().is_empty());
    }

    #[test]
    fn os_name_is_windows() {
        let os = system::os::get_os();
        assert_eq!(os, "Windows");
    }

    #[test]
    fn distro_string_not_empty() {
        let s = system::distro::get_distro(DistroDisplay::NameModelVersionArch);
        assert!(!s.trim().is_empty());
    }

    #[test]
    fn system_model_may_be_none_but_not_empty_when_some() {
        let m = system::model::get_model();
        if let Some(mm) = m {
            assert!(!mm.trim().is_empty());
        }
    }

    #[test]
    fn cpu_string_is_returned() {
        let s = info::cpu::get_cpu(
            /*cpu_brand*/ true,
            /*show_speed*/ true,
            /*show_cores*/ true,
            /*show_temp*/ false, // robust in environments without temp
            /*speed_shorthand*/ true,
            /*temp_unit*/ None,
        );
        assert!(s.is_some());
        assert!(!s.unwrap().trim().is_empty());
    }

    #[test]
    fn gpus_list_non_empty() {
        let gpus = info::gpu::get_gpus();
        assert!(!gpus.is_empty());
        assert!(gpus.iter().all(|g| !g.trim().is_empty()));
    }

    #[test]
    fn memory_reports_value() {
        let mem = info::memory::get_memory(false, MemoryUnit::MiB);
        assert!(mem.is_some());
        assert!(mem.as_ref().unwrap().contains("MiB"));
    }

    #[test]
    fn uptime_reports_value() {
        let up = info::uptime::get_uptime(UptimeShorthand::Full);
        assert!(up.is_some());
        assert!(!up.unwrap().trim().is_empty());
    }

    #[test]
    fn os_age_reports_value() {
        let age = info::os_age::get_os_age(OsAgeShorthand::Full);
        assert!(age.is_some());
        assert!(!age.unwrap().trim().is_empty());
    }

    #[test]
    fn disks_present_and_queryable() {
        let disks = info::disk::get_disks(DiskSubtitle::Dir, DiskDisplay::InfoBar, None);
        assert!(disks.is_some(), "Should detect at least one fixed drive");
        let list = disks.unwrap();
        assert!(!list.is_empty());
        for (label, val) in list {
            assert!(!label.trim().is_empty());
            assert!(!val.trim().is_empty());
        }
    }

    #[test]
    fn battery_query_should_not_panic() {
        // Many Windows environments (desktops/VMs) have no battery; allow empty vector.
        let _ = info::battery::get_battery(BatteryDisplayMode::InfoBar);
    }

    #[test]
    fn resolution_query_should_not_panic() {
        let _ = desktop::resolution::get_resolution(false);
    }

    #[test]
    fn wm_and_de_queries_should_not_panic() {
        let wm = desktop::wm::get_wm();
        let _de = desktop::de::get_de(false, wm.as_deref());
    }

    #[test]
    fn theme_query_should_not_panic() {
        let _ = desktop::theme::get_theme(None);
    }

    #[test]
    fn shell_query_should_not_panic() {
        let _ = shell::get_shell(false, false);
    }

    #[test]
    fn packages_query_should_not_panic() {
        let _ = packages::get_packages(PackageShorthand::On);
    }

    #[test]
    fn titles_are_non_empty() {
        let (user, host) = title::get_titles(false);
        assert!(!user.trim().is_empty());
        assert!(!host.trim().is_empty());
    }
}
