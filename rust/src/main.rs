extern crate lib;

// use lib::audio_engine;
use lib::media;

fn main() {
    media::read_flac("../media/techno.flac".to_string());
    // let graph = audio_engine::construct_audio_graph();
    // audio_engine::connect_to_output(graph);
}
