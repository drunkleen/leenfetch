// #[cfg(target_os = "freebsd")]
// pub mod freebsd;
#[cfg(target_os = "linux")]
pub mod linux;
// #[cfg(target_os = "macos")]
// pub mod macos;
#[cfg(target_os = "windows")]
pub mod windows;

// Re-export
#[cfg(target_os = "freebsd")]
pub use freebsd::*;
#[cfg(target_os = "linux")]
pub use linux::*;
#[cfg(target_os = "macos")]
pub use macos::*;
#[cfg(target_os = "windows")]
pub use windows::*;

pub mod ascii;
pub mod colors;
pub mod enums;
pub mod helper;
pub mod utils;
