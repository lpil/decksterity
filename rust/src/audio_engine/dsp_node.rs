use super::dsp;
use super::dsp::{Frame, FromSample, Node, Sample};

pub type Phase = f64;
pub type Volume = f32;
pub type Frequency = f64;

type OutBuffers = [super::Output; super::CHANNELS];

#[derive(Debug)]
pub enum DspNode {
    Master,
    Volume(f32),
    Oscillator(Phase, Frequency, Volume),
}

impl Node<OutBuffers> for DspNode {
    fn audio_requested(&mut self, buffer: &mut [OutBuffers], sample_hz: f64) {
        match *self {
            DspNode::Master => (),
            DspNode::Volume(v) => volume(buffer, v),
            DspNode::Oscillator(ref mut p, f, v) => oscillator(buffer, sample_hz, p, f, v),
        }
    }
}

fn oscillator(
    buffer: &mut [OutBuffers],
    sample_hz: f64,
    phase: &mut Phase,
    frequency: Frequency,
    volume: Volume,
) {
    dsp::slice::map_in_place(buffer, |_| {
        let val = sine_wave(*phase, volume);
        *phase += frequency / sample_hz;
        Frame::from_fn(|_| val)
    });
}

fn volume(buffer: &mut [OutBuffers], volume: Volume) {
    dsp::slice::map_in_place(buffer, |frame| frame.map(|s| s.mul_amp(volume)))
}

/// Return a sine wave for the given phase.
fn sine_wave<S: Sample>(phase: Phase, volume: Volume) -> S
where
    S: Sample + FromSample<f32>,
{
    use std::f64::consts::PI;
    ((phase * PI * 2.0).sin() as f32 * volume).to_sample::<S>()
}
