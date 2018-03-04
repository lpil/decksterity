use super::dsp;
use super::dsp::{Frame, Node, Sample};

pub type Phase = f64;
pub type Pitch = f64;

#[derive(Debug)]
pub enum DspNode {
    Master,
    Volume(f32),
    Player(Phase, Pitch, Vec<super::Frame>),
}

impl Node<super::Frame> for DspNode {
    fn audio_requested(&mut self, buffer: &mut [super::Frame], _sample_hz: f64) {
        match *self {
            DspNode::Master => {
                // Nothing here...
            }
            DspNode::Volume(volume) => {
                dsp::slice::map_in_place(buffer, |frame| frame.map(|s| s.mul_amp(volume)))
            }
            DspNode::Player(ref mut phase, pitch, ref samples) => {
                dsp::slice::map_in_place(buffer, |_| {
                    let frame = samples[*phase as usize];
                    // Why is the input audio SUPER loud?
                    let quiet_frame = frame.map(|s| s * 0.0001);
                    *phase += pitch;
                    quiet_frame
                })
            }
        }
    }
}
