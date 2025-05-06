use libc::{c_char, c_int, c_ulong};
use std::ptr;
use std::{env, fs};

#[repr(C)]
struct XRRScreenConfiguration {
    _private: [u8; 0],
}

#[link(name = "X11")]
#[link(name = "Xrandr")]
extern "C" {
    fn XOpenDisplay(display_name: *const c_char) -> *mut Display;
    fn XCloseDisplay(display: *mut Display);
    fn XDefaultScreen(display: *mut Display) -> c_int;
    fn XDisplayWidth(display: *mut Display, screen_number: c_int) -> c_int;
    fn XDisplayHeight(display: *mut Display, screen_number: c_int) -> c_int;

    fn XRRGetScreenInfo(display: *mut Display, window: Window) -> *mut XRRScreenConfiguration;
    fn XRRFreeScreenConfigInfo(config: *mut XRRScreenConfiguration);
    fn XRRConfigCurrentRate(config: *mut XRRScreenConfiguration) -> i16;
    fn XRootWindow(display: *mut Display, screen_number: c_int) -> Window;
}

#[repr(C)]
pub struct Display {
    _private: [u8; 0],
}
type Window = c_ulong;

pub fn get_resolution(refresh_rate: bool) -> Option<String> {
    // X11 path
    if env::var("DISPLAY").is_ok() {
        if let Some(res) = try_x11(refresh_rate) {
            return Some(res);
        }
    }

    // DRM/KMS path
    if let Some(res) = try_drm(refresh_rate) {
        return Some(res);
    }

    // Framebuffer path
    if let Some(res) = try_fb() {
        return Some(res);
    }

    // Wayland path (unsupported without compositor protocol)
    if env::var("WAYLAND_DISPLAY").is_ok() {
        return Some("Wayland: resolution unavailable (restricted by compositor)".to_string());
    }

    None
}

fn try_x11(refresh_rate: bool) -> Option<String> {
    unsafe {
        let display = XOpenDisplay(ptr::null());
        if display.is_null() {
            return None;
        }

        let screen = XDefaultScreen(display);
        let width = XDisplayWidth(display, screen);
        let height = XDisplayHeight(display, screen);
        let mut result = format!("{}x{}", width, height);

        if refresh_rate {
            let root = XRootWindow(display, screen);
            let config = XRRGetScreenInfo(display, root);
            if !config.is_null() {
                let rate = XRRConfigCurrentRate(config);
                result = format!("{} @ {}Hz", result, rate);
                XRRFreeScreenConfigInfo(config);
            }
        }

        XCloseDisplay(display);
        Some(result)
    }
}

fn try_drm(refresh_rate: bool) -> Option<String> {
    let entries = fs::read_dir("/sys/class/drm/").ok()?;

    for entry in entries.flatten() {
        let path = entry.path();
        if path.file_name()?.to_str()?.contains("card") && path.join("status").exists() {
            let status = fs::read_to_string(path.join("status")).ok()?;
            if status.trim() != "connected" {
                continue;
            }

            let mode_path = path.join("modes");
            if mode_path.exists() {
                let mode = fs::read_to_string(mode_path)
                    .ok()?
                    .lines()
                    .next()?
                    .to_string();
                return Some(if refresh_rate {
                    format!("{} @ 60Hz", mode) // Fallback rate assumption
                } else {
                    mode
                });
            }
        }
    }

    None
}

fn try_fb() -> Option<String> {
    let contents = fs::read_to_string("/sys/class/graphics/fb0/virtual_size").ok()?;
    let (w, h) = contents.trim().split_once(',')?;
    Some(format!("{}x{}", w.trim(), h.trim()))
}
