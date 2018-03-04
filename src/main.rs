extern crate cpal;
extern crate dsp;
extern crate lib;

pub const CHANNELS: usize = 2;
pub const SAMPLE_HZ: f64 = 44_100.0;
pub type Sample = f32;
pub type Frame = [Sample; CHANNELS];
use std::{thread, time};
use std::sync::{Arc, Mutex};
use lib::{audio_engine, media};

use self::dsp::{Graph, Node};
use self::dsp::sample::ToFrameSliceMut;
use self::lib::audio_engine::dsp_node::DspNode;

fn main() {
    let media = media::read_flac("./media/short-techno.flac".to_string());
    let mut engine = audio_engine::new();
    engine.set_media(media);

    let engine_arc = Arc::new(Mutex::new(engine));

    {
        let engine = Arc::clone(&engine_arc);
        thread::spawn(move || loop {
            for i in [0.9, 1.0, 1.1, 1.0].iter() {
                let pause = time::Duration::from_millis(2000);
                thread::sleep(pause);
                engine.lock().unwrap().set_pitch(*i);
            }
        });
    }

    // TODO: Put this in lib somehow.

    let device = cpal::default_output_device().expect("Failed to get default output device");
    let format = device
        .default_output_format()
        .expect("Failed to get default output format");
    let event_loop = cpal::EventLoop::new();
    let stream_id = event_loop.build_output_stream(&device, &format).unwrap();
    event_loop.play_stream(stream_id.clone());

    event_loop.run(move |_, data| {
        let b = match data {
            cpal::StreamData::Output { buffer: b } => b,
            _ => return,
        };

        match b {
            cpal::UnknownTypeOutputBuffer::F32(mut buffer) => {
                let raw_buffer: &mut [Frame] = buffer.to_frame_slice_mut().unwrap();
                dsp::slice::equilibrium(raw_buffer);
                engine_arc
                    .lock()
                    .unwrap()
                    .graph
                    .audio_requested(raw_buffer, SAMPLE_HZ);
            }
            // TODO: Other output formats
            _ => return,
        }
    });
}
