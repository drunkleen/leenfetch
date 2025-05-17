use std::process::Command;

use crate::modules::enums::SongInfo;

pub fn get_song() -> Option<SongInfo> {
    let player = detect_player()?;

    let output = match player.as_str() {
        "spotify" => run_powershell(&[
            "powershell",
            "-Command",
            "(Get-Process -Name spotify -ErrorAction SilentlyContinue).MainWindowTitle",
        ]),
        _ => None,
    }?;

    parse_song_output(&output)
}

fn parse_song_output(output: &str) -> Option<SongInfo> {
    // Spotify's window title is usually "Artist - Song"
    let trimmed = output.trim();
    let (artist, title) = trimmed.split_once(" - ")?;

    Some(SongInfo {
        artist: artist.to_string(),
        album: "Unknown Album".into(),
        title: title.to_string(),
    })
}

fn run_powershell(cmd: &[&str]) -> Option<String> {
    let output = Command::new(cmd[0]).args(&cmd[1..]).output().ok()?;
    if !output.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&output.stdout).to_string())
}

fn detect_player() -> Option<String> {
    let players = ["Spotify", "vlc", "groove"];

    let output = Command::new("tasklist").output().ok()?;
    let list = String::from_utf8_lossy(&output.stdout);

    for player in players {
        if list.contains(player) {
            return Some(player.to_lowercase());
        }
    }

    None
}
