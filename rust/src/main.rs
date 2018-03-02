extern crate cpal;
extern crate dsp;

/// SoundStream is currently generic over i8, i32 and f32. Feel free to change it!
type Output = f32;

type Phase = f64;
type Frequency = f64;
type Volume = f32;

const CHANNELS: usize = 2;
const SAMPLE_HZ: f64 = 44_100.0;

const A5_HZ: Frequency = 440.0;
const D5_HZ: Frequency = 587.33;
const F5_HZ: Frequency = 698.46;

use dsp::{Frame, FromSample, Graph, Node, Sample};
use dsp::sample::ToFrameSliceMut;

fn main() {
    //
    // Construct audio output
    //

    let device = cpal::default_output_device().expect("Failed to get default output device");
    let format = device
        .default_output_format()
        .expect("Failed to get default output format");
    let event_loop = cpal::EventLoop::new();
    let stream_id = event_loop.build_output_stream(&device, &format).unwrap();
    event_loop.play_stream(stream_id.clone());

    //
    // Construct our dsp graph.
    //

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

    // The callback we'll use to pass to the Stream. It will request audio from our dsp_graph.
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
            _ => return,
        }
    });
}

/// Our type for which we will implement the `Dsp` trait.
#[derive(Debug)]
enum DspNode {
    Master,
    Volume(f32),
    Oscillator(Phase, Frequency, Volume),
}

impl Node<[Output; CHANNELS]> for DspNode {
    /// Here we'll override the audio_requested method and generate a sine wave.
    fn audio_requested(&mut self, buffer: &mut [[Output; CHANNELS]], sample_hz: f64) {
        match *self {
            DspNode::Master => (),
            DspNode::Volume(volume) => {
                dsp::slice::map_in_place(buffer, |frame| frame.map(|s| s.mul_amp(volume)))
            }
            DspNode::Oscillator(ref mut phase, frequency, volume) => {
                dsp::slice::map_in_place(buffer, |_| {
                    let val = sine_wave(*phase, volume);
                    *phase += frequency / sample_hz;
                    Frame::from_fn(|_| val)
                });
            }
        }
    }
}

/// Return a sine wave for the given phase.
fn sine_wave<S: Sample>(phase: Phase, volume: Volume) -> S
where
    S: Sample + FromSample<f32>,
{
    use std::f64::consts::PI;
    ((phase * PI * 2.0).sin() as f32 * volume).to_sample::<S>()
}
