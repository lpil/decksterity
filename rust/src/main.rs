extern crate lib;

// use lib::audio_engine;
use lib::media;

fn main() {
    let _media = media::read_flac("../media/short-techno.flac".to_string());
    // let graph = audio_engine::construct_audio_graph();
    // audio_engine::connect_to_output(graph);
}
