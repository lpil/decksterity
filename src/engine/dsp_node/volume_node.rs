use dsp::{Frame, Node, Sample};
use super::super::dsp;
use super::super::super::engine;

#[derive(Debug)]
pub struct VolumeNode {
    amount: f32,
}

impl VolumeNode {
    pub fn new() -> Self {
        Self { amount: 0.0 }
    }

    pub fn set_volume(&mut self, new_amount: f32) -> f32 {
        self.amount = new_amount;
        new_amount
    }
}

impl Node<engine::Frame> for VolumeNode {
    fn audio_requested(&mut self, out_buffer: &mut [engine::Frame], _sample_hz: f64) {
        dsp::slice::map_in_place(out_buffer, |frame| frame.map(|s| s.mul_amp(self.amount)))
    }
}
