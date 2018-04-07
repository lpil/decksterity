use super::library::Track;
use toml;
use std::{convert, fs};
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::collections::HashMap;

#[derive(Debug)]
pub enum Error {
    IOFail(io::Error),
    ParseFail(toml::de::Error),
    SerializeFail(toml::ser::Error),
}

impl convert::From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IOFail(err)
    }
}

impl convert::From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Error::ParseFail(err)
    }
}

impl convert::From<toml::ser::Error> for Error {
    fn from(err: toml::ser::Error) -> Self {
        Error::SerializeFail(err)
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct TracksState {
    version: u8,
    tracks: HashMap<String, Track>,
}

fn data_dir() -> String {
    "/home/louis/.local/share/decksterity".to_string()
}

fn tracks_file() -> String {
    format!("{}/tracks.toml", data_dir())
}

pub fn write_tracks_state(tracks: HashMap<String, Track>) -> Result<(), Error> {
    let state = TracksState { version: 1, tracks };
    fs::create_dir_all(data_dir())?;
    File::create(tracks_file())?.write_all(toml::to_string(&state)?.as_bytes())?;
    Ok(())
}

pub fn load_tracks_state() -> Result<HashMap<String, Track>, Error> {
    let mut toml = String::new();
    File::open(tracks_file())?.read_to_string(&mut toml)?;
    let state: TracksState = toml::from_str(&toml)?;
    Ok(state.tracks)
}
