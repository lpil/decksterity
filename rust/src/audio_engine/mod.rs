extern crate cpal;
extern crate dsp;

mod dsp_node;

use self::dsp::{Graph, Node};
use self::dsp::sample::ToFrameSliceMut;
use self::dsp_node::DspNode;

pub type Output = f32;

pub type AudioEngine = dsp::Graph<[Output; CHANNELS], DspNode>;

pub const CHANNELS: usize = 2;
pub const SAMPLE_HZ: f64 = 44_100.0;

const A5_HZ: dsp_node::Frequency = 440.0;
const D5_HZ: dsp_node::Frequency = 587.33;
const F5_HZ: dsp_node::Frequency = 698.46;

pub fn construct_audio_graph() -> AudioEngine {
    let mut graph = Graph::new();

    let master = graph.add_node(DspNode::Master);
    let master_vol = graph.add_node(DspNode::Volume(1.0));

    graph
        .add_connection(master_vol, master)
        .expect("feedback loop");

    graph.add_input(DspNode::Oscillator(0.0, A5_HZ, 0.2), master_vol);
    graph.add_input(DspNode::Oscillator(0.0, D5_HZ, 0.1), master_vol);
    graph.add_input(DspNode::Oscillator(0.0, F5_HZ, 0.15), master_vol);

    graph.set_master(Some(master));
    graph
}

pub fn connect_to_output(mut graph: AudioEngine) {
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
                let raw_buffer: &mut [[Output; CHANNELS]] = buffer.to_frame_slice_mut().unwrap();
                dsp::slice::equilibrium(raw_buffer);
                graph.audio_requested(raw_buffer, SAMPLE_HZ);
            }
            // TODO: Other output formats
            _ => return,
        }
    });
}
