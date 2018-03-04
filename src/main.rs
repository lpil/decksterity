extern crate lib;

use lib::audio_engine;
use lib::media;

fn main() {
    let media = media::read_flac("./media/short-techno.flac".to_string());
    let mut engine = audio_engine::construct_audio_graph();
    engine.set_deck_media(media);
    engine.connect_to_output();
}
