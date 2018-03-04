use super::dsp;
use super::dsp::{Frame, FromSample, Node, Sample};

pub type Phase = f64;
pub type Pitch = f64;
pub type Volume = f32;
pub type Frequency = f64;

#[derive(Debug)]
pub enum DspNode {
    Master,
    Volume(f32),
    Oscillator(Phase, Frequency, Volume),
    Player(Phase, Pitch, Vec<super::Frame>),
}

impl Node<super::Frame> for DspNode {
    fn audio_requested(&mut self, buffer: &mut [super::Frame], sample_hz: f64) {
        match *self {
            DspNode::Master => {
                // Nothing here...
            }
            DspNode::Volume(volume) => {
                dsp::slice::map_in_place(buffer, |frame| frame.map(|s| s.mul_amp(volume)))
            }
            DspNode::Oscillator(ref mut phase, frequency, volume) => {
                dsp::slice::map_in_place(buffer, |_| {
                    let val = sine_wave(*phase, volume);
                    *phase += frequency / sample_hz;
                    Frame::from_fn(|_| val)
                })
            }
            DspNode::Player(_phase, _pitch, ref _samples) => {
                // Not yet!
                ()
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
