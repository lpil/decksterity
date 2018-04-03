use walkdir::WalkDir;
use regex::Regex;
use std::path::{Path, PathBuf};

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
        library_base_path: "/home/louis/data/new-music".to_string(),
    }
}

pub fn scan() {
    println!("Starting scan...");
    let config = load_config();

    let files: Vec<_> = WalkDir::new(config.library_base_path.clone())
        .follow_links(true)
        .into_iter()
        .take(5)
        .filter_map(|result| result.ok())
        .map(|dir_entry| dir_entry.path().to_path_buf())
        .filter_map(|entry| to_track(&entry, &config))
        .collect();

    println!("{:?}", files);
    println!("Done");
}

lazy_static! {
    static ref TRACK_REGEX: Regex = Regex::new(
        r"(?i)((?P<number>\d+) )?((?P<artist>[^/]+) - )?(?P<title>[^/]+)\.(?P<extension>flac|mp3|ogg|wav)$")
        .expect("Invalid Regex: TRACK_REGEX");
}

fn to_track(path_buf: &PathBuf, config: &Config) -> Option<Track> {
    let prefix = &config.library_base_path;
    let relative_path = path_buf.strip_prefix(Path::new(prefix)).ok()?;

    let captures = TRACK_REGEX.captures(path_buf.to_str()?)?;

    let track = Track {
        title: captures.name("title").map(|e| e.as_str().to_string()),
        artist: captures.name("artist").map(|e| e.as_str().to_string()),
        bpm: None,
        number: captures
            .name("number")
            .and_then(|e| e.as_str().to_string().parse().ok()),
        path: relative_path.to_path_buf(),
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

        assert_eq!(track.title, Some("Rip your Nips off (180 BPM)".to_string())); // TODO: Split out BPM
        assert_eq!(track.artist, Some("Captain Credible".to_string()));
        assert_eq!(track.number, Some(1));
        assert_eq!(track.bpm, None);
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
        assert_eq!(track.path, &pb("Captain Credible - Rip your Nips off.flac"));

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
