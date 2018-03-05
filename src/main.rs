extern crate dsp;
extern crate jack;
extern crate lib;

use std::{thread, time};
use std::sync::{Arc, Mutex};
use lib::{audio_engine, media};
use dsp::Node;

const MASTER_AMP: f32 = 0.01;

fn main() {
    // Set up deck
    let media = media::read_flac("./media/short-techno.flac".to_string());
    let mut engine = audio_engine::new();
    engine.set_media(media);
    let engine_arc = Arc::new(Mutex::new(engine));

    // Set up MIDI input
    // _conn_in needs to be a named parameter because it needs to be kept
    // alive until the end of the scope
    let _conn_in = lib::midi::connect_xone_k2(Arc::clone(&engine_arc));

    // Start your engines
    let (client, _status) =
        jack::Client::new("decksterity", jack::ClientOptions::NO_START_SERVER).unwrap();

    let mut out_left = client
        .register_port("decksterity-a-left", jack::AudioOut::default())
        .unwrap();
    let mut out_right = client
        .register_port("decksterity-a-right", jack::AudioOut::default())
        .unwrap();

    let buffer_size = client.buffer_size() as usize;
    let sample_rate = client.sample_rate();
    let mut frame_buffer = Vec::new();
    frame_buffer.resize(buffer_size, [0.0, 0.0]);

    let process = jack::ClosureProcessHandler::new(
        move |_: &jack::Client, ps: &jack::ProcessScope| -> jack::Control {
            let out_buffer_left = out_left.as_mut_slice(ps);
            let out_buffer_right = out_right.as_mut_slice(ps);

            // Minimise the scope in which the engine is locked
            {
                engine_arc
                    .lock()
                    .unwrap()
                    .graph
                    .audio_requested(&mut frame_buffer, sample_rate as f64);
            }

            // Is there any way we can avoid this extra copy?
            // The graph.audio_requested function expects a slice of frames,
            // but with JACK we can only offer multiple slices of samples.
            // Because of this we have to construct the `frame_buffer` above
            // and then copy from that after the graph has written to it.
            for i in 0..buffer_size {
                let frame = frame_buffer[i];
                out_buffer_left[i] = frame[0] * MASTER_AMP;
                out_buffer_right[i] = frame[1] * MASTER_AMP;
            }

            jack::Control::Continue
        },
    );

    // _jack_client needs to be a named parameter because it needs to be kept
    // alive until the end of the scope
    let _jack_client = client.activate_async((), process).unwrap();

    loop {
        thread::sleep(time::Duration::from_millis(24 * 60 * 60 * 1000));
    }
}
