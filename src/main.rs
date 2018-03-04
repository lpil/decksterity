extern crate lib;

use lib::{audio_engine, media};
// use std::{thread, time};

fn main() {
    let media = media::read_flac("./media/short-techno.flac".to_string());
    let mut engine = audio_engine::new();
    engine.set_media(media);
    engine.set_pitch(1.5);

    engine.connect_to_output();

    // println!("sleeping");
    // let pause = time::Duration::from_millis(500);
    // thread::sleep(pause);

    // println!("setting pitch");
    // engine.set_pitch(1.5);
    // println!("pitch set");
}
