use std::mem;
use super::super::{dsp, media};
use dsp::{Frame, Node};
use super::super::super::audio_engine;

#[derive(Debug)]
pub struct PlayerNode {
    is_playing: bool,
    offset: f64,
    pitch: f64,
    buffer: Vec<audio_engine::Frame>,
}

impl PlayerNode {
    pub fn new() -> Self {
        Self {
            is_playing: false,
            offset: 0.0,
            pitch: 1.0,
            buffer: vec![],
        }
    }

    pub fn toggle_play_pause(&mut self) -> bool {
        let Self {
            ref mut is_playing, ..
        } = *self;
        let new_value = !*is_playing;
        *is_playing = new_value;
        new_value
    }

    pub fn adjust_pitch(&mut self, delta: f64) -> f64 {
        let Self { ref mut pitch, .. } = *self;
        let new_pitch = *pitch + delta;
        *pitch = new_pitch;
        new_pitch
    }

    pub fn set_media(&mut self, media: media::Media) {
        let Self {
            ref mut offset,
            ref mut buffer,
            ..
        } = *self;
        mem::replace(buffer, media);
        *offset = 0.0;
    }

    //     pub fn set_pitch(&mut self, new_pitch: f64) -> f64 {
    //         let Self { ref mut pitch, .. } = *self;
    //         *pitch = new_pitch;
    //         new_pitch
    //     }
}

impl Node<audio_engine::Frame> for PlayerNode {
    fn audio_requested(&mut self, out_buffer: &mut [audio_engine::Frame], _sample_hz: f64) {
        if !self.is_playing {
            return; // When not playing the Player does nothing.
        }

        let Self {
            ref mut offset,
            pitch,
            ref buffer,
            ..
        } = *self;

        dsp::slice::map_in_place(out_buffer, |_| {
            let frame = buffer.get(*offset as usize).unwrap_or_else(|| {
                *offset = 0.0;
                &buffer[0]
            });

            // Why is the input audio SUPER loud?
            let quiet_frame = frame.map(|s| s * 0.00003);
            *offset += pitch;
            quiet_frame
        })
    }
}
