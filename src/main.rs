extern crate cpal;
extern crate dsp;
extern crate lib;

use std::{thread, time};
use std::sync::{Arc, RwLock};
use lib::{audio_engine, media};

fn main() {
    let media = media::read_flac("./media/short-techno.flac".to_string());
    let mut engine = audio_engine::new();
    engine.set_media(media);

    let engine_arc = Arc::new(RwLock::new(engine));

    {
        let engine = Arc::clone(&engine_arc);
        let mut i = 1.0;
        let mut step = 0.025;
        thread::spawn(move || loop {
            if i <= 0.2 || i >= 1.0 {
                step = 0.0 - step;
            };

            i = i + step;
            println!("Setting pitch to {}", i);
            engine.write().unwrap().set_pitch(i);
            thread::sleep(time::Duration::from_millis(250));
        });
    }

    audio_engine::connect_to_output(engine_arc);
}
