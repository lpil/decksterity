use super::dsp;
use super::dsp::{Frame, Node, Sample};

pub type IsPlaying = bool;
pub type Offset = f64;
pub type Pitch = f64;
pub type Amp = f32;

#[derive(Debug)]
pub enum DspNode {
    Master,
    Volume(Amp),
    Player(IsPlaying, Offset, Pitch, Vec<super::Frame>),
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
            DspNode::Player(false, _, _, _) => {
                // When not playing the Player does nothing.
            }
            DspNode::Player(true, ref mut offset, pitch, ref samples) => {
                dsp::slice::map_in_place(buffer, |_| {
                    let frame = samples.get(*offset as usize).unwrap_or_else(|| {
                        *offset = 0.0;
                        &samples[0]
                    });

                    // Why is the input audio SUPER loud?
                    let quiet_frame = frame.map(|s| s * 0.00003);
                    *offset += pitch;
                    quiet_frame
                })
            }
        }
    }
}

pub fn new_player() -> DspNode {
    DspNode::Player(false, 0.0, 1.0, vec![])
}
