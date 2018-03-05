extern crate cpal;
extern crate dsp;
extern crate lib;

use std::sync::{Arc, RwLock};
use lib::{audio_engine, media};

fn main() {
    // Set up deck
    let media = media::read_flac("./media/short-techno.flac".to_string());
    let mut engine = audio_engine::new();
    engine.set_media(media);
    let engine_arc = Arc::new(RwLock::new(engine));

    // Set up MIDI input
    // _conn_in needs to be a named parameter because it needs to be kept
    // alive until the end of the scope
    let _conn_in = lib::midi::connect_xone_k2(Arc::clone(&engine_arc));

    // Start your engines
    audio_engine::connect_to_output(engine_arc);
}
