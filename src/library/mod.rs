use walkdir::WalkDir;
use regex::Regex;

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

    let files: Vec<_> = WalkDir::new(config.library_base_path)
        .follow_links(true)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.file_name().to_str().unwrap().to_string())
        .filter(|p| has_audio_file_extension(&*p))
        .collect();

    println!("{:?}", files);
    println!("Done");
}

lazy_static! {
    static ref AUDIO_FILE_EXTENSION_REGEX: Regex = Regex::new("(?i)\\.(flac|mp3|ogg|wav)$")
        .expect("Invalid Regex: AUDIO_FILE_EXTENSION_REGEX");
}

fn has_audio_file_extension(path: &str) -> bool {
    AUDIO_FILE_EXTENSION_REGEX.is_match(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_audio_file_extension_test() {
        assert!(has_audio_file_extension("foo.flac"));
        assert!(has_audio_file_extension("foo.mp3"));
        assert!(has_audio_file_extension("foo.ogg"));
        assert!(has_audio_file_extension("foo.wav"));
        assert!(has_audio_file_extension("FOO.FLAC"));
        assert!(has_audio_file_extension("FOO.MP3"));
        assert!(has_audio_file_extension("FOO.OGG"));
        assert!(has_audio_file_extension("FOO.WAV"));
        assert!(!has_audio_file_extension("foo.png"));
        assert!(!has_audio_file_extension("foo.jpg"));
        assert!(!has_audio_file_extension("foo"));
        assert!(!has_audio_file_extension("foo.waver"));
    }
}
