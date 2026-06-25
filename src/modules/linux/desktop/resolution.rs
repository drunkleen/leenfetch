use std::fs;

pub fn get_resolution() -> Option<String> {
    // DRM/KMS path
    if let Some(res) = try_drm() {
        return Some(res);
    }

    // Framebuffer path
    if let Some(res) = try_fb() {
        return Some(res);
    }

    // Wayland path (unsupported without compositor protocol)
    if std::env::var("WAYLAND_DISPLAY").is_ok() {
        return Some("Wayland: resolution unavailable (restricted by compositor)".to_string());
    }

    None
}

fn try_drm() -> Option<String> {
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
                return Some(mode);
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
