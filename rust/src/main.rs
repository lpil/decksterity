extern crate lib;

use lib::audio_engine;

fn main() {
    let graph = audio_engine::construct_audio_graph();
    audio_engine::connect_to_output(graph);
}
