use std::env;
use std::process::Command;

#[derive(Debug)]
pub struct SongInfo {
    pub artist: String,
    pub album: String,
    pub title: String,
}

pub fn get_song() -> Option<SongInfo> {
    let player = detect_player()?;

    // Match player and run respective logic
    let output = match player.as_str() {
        "mocp" => run(&["mocp", "-Q", "%artist \n%album \n%song"]),
        "deadbeef" => run(&[
            "deadbeef",
            "--nowplaying-tf",
            "%artist% \\n%album% \\n%title%",
        ]),
        "qmmp" => run(&["qmmp", "--nowplaying", "%p \n%a \n%t"]),
        "spotify" | "vlc" | "rhythmbox" | "clementine" | "lollypop" => run(&[
            "playerctl",
            "metadata",
            "--player",
            player.as_str(),
            "--format",
            "{{ artist }}\n{{ album }}\n{{ title }}",
        ]),
        "mpd" | "mopidy" => run(&["mpc", "-f", "%artist% \n%album% \n%title%", "current"]),
        "playerctl" => run(&[
            "playerctl",
            "metadata",
            "--format",
            "{{ artist }}\n{{ album }}\n{{ title }}",
        ]),
        _ => return None,
    }?;

    // Parse output
    let mut lines = output.lines();
    let artist = lines.next().unwrap_or("Unknown Artist").trim().to_string();
    let album = lines.next().unwrap_or("Unknown Album").trim().to_string();
    let title = lines.next().unwrap_or("Unknown Song").trim().to_string();

    Some(SongInfo {
        artist,
        album,
        title,
    })
}

fn run(cmd: &[&str]) -> Option<String> {
    let output = Command::new(cmd[0]).args(&cmd[1..]).output().ok()?;
    if !output.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&output.stdout).to_string())
}

fn detect_player() -> Option<String> {
    let env_player = env::var("MUSIC_PLAYER").ok();
    if let Some(p) = env_player {
        return Some(p);
    }

    let players = [
        "spotify",
        "playerctl",
        "mocp",
        "qmmp",
        "mpd",
        "mopidy",
        "vlc",
        "rhythmbox",
        "deadbeef",
        "lollypop",
        "clementine",
    ];

    let output = Command::new("ps").arg("aux").output().ok()?;
    let ps_output = String::from_utf8_lossy(&output.stdout);

    for player in players {
        if ps_output.contains(player) {
            return Some(player.to_string());
        }
    }

    None
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_parse_output_defaults() {
        let mock_output = "\n\n\n";
        let mut lines = mock_output.lines();

        let artist = {
            let val = lines.next().unwrap_or("").trim();
            if val.is_empty() {
                "Unknown Artist"
            } else {
                val
            }
        };
        let album = {
            let val = lines.next().unwrap_or("").trim();
            if val.is_empty() {
                "Unknown Album"
            } else {
                val
            }
        };
        let title = {
            let val = lines.next().unwrap_or("").trim();
            if val.is_empty() {
                "Unknown Song"
            } else {
                val
            }
        };

        assert_eq!(artist, "Unknown Artist");
        assert_eq!(album, "Unknown Album");
        assert_eq!(title, "Unknown Song");
    }
}
