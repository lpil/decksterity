use walkdir::WalkDir;
use regex::Regex;
use std::path::{Path, PathBuf};
use toml;
use std::fs;
use std::fs::File;
use std::io::Write;

use std::collections::HashMap;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct TracksState {
    version: u8,
    tracks: HashMap<String, Track>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct Track {
    title: Option<String>,
    artist: Option<String>,
    number: Option<u8>,
    bpm: Option<f32>,
    path: PathBuf,
}

struct Config {
    library_base_path: String,
}

/// Load config from TOML file... Or at least at will. Use a dummy for now.
///
fn load_config() -> Config {
    Config {
        library_base_path: "/home/louis/data/new-music/".to_string(),
    }
}

pub fn scan() {
    println!("Starting scan...");
    let config = load_config();

    let mut tracks = HashMap::new();

    WalkDir::new(config.library_base_path.clone())
        .follow_links(true)
        .into_iter()
        .filter_map(|result| result.ok())
        .map(|dir_entry| dir_entry.path().to_path_buf())
        .filter_map(|entry| to_track(&entry, &config))
        .for_each(|track| {
            tracks.insert(track.path.to_str().unwrap().to_string(), track);
        });

    let tracks_state = TracksState { version: 1, tracks };

    let toml = toml::to_string(&tracks_state).unwrap();

    let data_dir = "/home/louis/.local/share/decksterity";
    fs::create_dir_all(data_dir).expect(&format!("Unable to create {}", data_dir));

    let tracks_file = format!("{}/tracks.toml", data_dir);
    File::create(&tracks_file)
        .expect("Unable to create tracks file")
        .write_all(toml.as_bytes())
        .expect("Unable to write tracks file");
}

lazy_static! {
    static ref TRACK_REGEX: Regex = Regex::new(
        r"(?i)((?P<number>\d+) )?((?P<artist>[^/]+) - )?(?P<title>[^/]+)\.(?P<extension>flac|mp3|ogg|wav)$")
        .expect("Invalid Regex: TRACK_REGEX");

    static ref TRACK_BPM_REGEX: Regex = Regex::new(
        r"(?i)(?P<title>.+) \((?P<bpm>\d+(\.\d+)?) BPM\)")
        .expect("Invalid Regex: TRACK_REGEX");
}

fn extract_bpm(full_title: &String) -> Option<(String, f32)> {
    let title_captures = TRACK_BPM_REGEX.captures(full_title)?;

    let title = title_captures
        .name("title")
        .map(|e| e.as_str().to_string())?;

    let bpm = title_captures
        .name("bpm")
        .and_then(|e| e.as_str().to_string().parse().ok())?;

    Some((title, bpm))
}

fn to_track(path_buf: &PathBuf, config: &Config) -> Option<Track> {
    let prefix = &config.library_base_path;
    let relative_path = path_buf.strip_prefix(Path::new(prefix)).ok()?;

    let captures = TRACK_REGEX.captures(path_buf.to_str()?)?;

    let full_title = captures.name("title").map(|e| e.as_str().to_string())?;
    let (title, bpm) = match extract_bpm(&full_title) {
        Some((title, bpm)) => (Some(title), Some(bpm)),
        None => (Some(full_title), None),
    };

    let artist = captures.name("artist").map(|e| e.as_str().to_string());
    let number = captures
        .name("number")
        .and_then(|e| e.as_str().to_string().parse().ok());
    let path = relative_path.to_path_buf();

    let track = Track {
        title,
        artist,
        bpm,
        number,
        path,
    };
    Some(track)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path, PathBuf};

    fn pb(s: &str) -> PathBuf {
        Path::new(s).to_path_buf()
    }

    #[test]
    fn to_track_test() {
        let cfg = Config {
            library_base_path: "etc".to_string(),
        };

        // These shouldn't be possible
        assert_eq!(to_track(&pb(""), &cfg), None);
        assert_eq!(to_track(&pb("no-root.flac"), &cfg), None);
        assert_eq!(to_track(&pb("other/root/foo.flac"), &cfg), None);

        // Full track
        let track = to_track(
            &pb("etc/01 Captain Credible - Rip your Nips off (180 BPM).flac"),
            &cfg,
        ).expect("etc/foo.flac should to_track ok");

        assert_eq!(track.title, Some("Rip your Nips off".to_string())); // TODO: Split out BPM
        assert_eq!(track.artist, Some("Captain Credible".to_string()));
        assert_eq!(track.number, Some(1));
        assert_eq!(track.bpm, Some(180.0));
        assert_eq!(
            track.path,
            pb("01 Captain Credible - Rip your Nips off (180 BPM).flac")
        );

        // Track minus BPM and number
        let track = to_track(&pb("etc/Captain Credible - Rip your Nips off.flac"), &cfg)
            .expect("etc/foo.flac should to_track ok");

        assert_eq!(track.title, Some("Rip your Nips off".to_string()));
        assert_eq!(track.artist, Some("Captain Credible".to_string()));
        assert_eq!(track.bpm, None);
        assert_eq!(track.path, pb("Captain Credible - Rip your Nips off.flac"));

        // Valid extensions
        assert!(to_track(&pb("etc/foo.flac"), &cfg).is_some());
        assert!(to_track(&pb("etc/foo.FLAC"), &cfg).is_some());
        assert!(to_track(&pb("etc/foo.wav"), &cfg).is_some());
        assert!(to_track(&pb("etc/foo.WAV"), &cfg).is_some());
        assert!(to_track(&pb("etc/foo.ogg"), &cfg).is_some());
        assert!(to_track(&pb("etc/foo.OGG"), &cfg).is_some());
        assert!(to_track(&pb("etc/foo.mp3"), &cfg).is_some());
        assert!(to_track(&pb("etc/foo.MP3"), &cfg).is_some());

        // Invalid extensions
        assert!(to_track(&pb("etc/foo.txt"), &cfg).is_none());
        assert!(to_track(&pb("etc/foo.png"), &cfg).is_none());
        assert!(to_track(&pb("etc/foo.jpg"), &cfg).is_none());
        assert!(to_track(&pb("etc/foo"), &cfg).is_none());
    }
}
