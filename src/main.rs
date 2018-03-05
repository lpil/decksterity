extern crate cpal;
extern crate dsp;
extern crate lib;

use std::io::Read;

use std::thread;
use std::sync::{Arc, RwLock};
use lib::{audio_engine, media};

fn main() {
    let media = media::read_flac("./media/short-techno.flac".to_string());
    let mut engine = audio_engine::new();
    engine.set_media(media);

    let engine_arc = Arc::new(RwLock::new(engine));

    {
        let engine = Arc::clone(&engine_arc);

        thread::spawn(move || loop {
            let c = std::io::stdin()
                .bytes()
                .next()
                .and_then(|result| result.ok())
                .map(|byte| byte as char);
            match c {
                Some('\n') => engine
                    .write()
                    .expect("unable to acquire write lock")
                    .toggle_play_pause(),
                _ => (),
            }
        });
    }

    audio_engine::connect_to_output(engine_arc);
}
