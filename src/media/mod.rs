use super::audio_engine;

use claxon;

pub type Media = Vec<audio_engine::Frame>;

pub fn read_flac(path: String) -> Media {
    let mut reader = claxon::FlacReader::open(path).expect("Unable to open FLAC file");
    let mut samples = reader.samples();
    let capacity = (44_100.0 * 60.0 * 10.0) as usize;
    let mut frames = Vec::with_capacity(capacity);

    while let (Some(Ok(l)), Some(Ok(r))) = (samples.next(), samples.next()) {
        frames.push([l as audio_engine::Sample, r as audio_engine::Sample]);
    }

    frames.shrink_to_fit();
    frames
}
