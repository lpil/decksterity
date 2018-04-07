mod player_node;
mod volume_node;

use super::dsp::Node;
use self::player_node::PlayerNode;
use self::volume_node::VolumeNode;

pub type Pitch = f64;
pub type Player = PlayerNode;
pub type Volume = VolumeNode;

#[derive(Debug)]
pub enum DspNode {
    Master,
    Volume(VolumeNode),
    Player(PlayerNode),
}

impl Node<super::Frame> for DspNode {
    fn audio_requested(&mut self, buffer: &mut [super::Frame], sample_hz: f64) {
        match *self {
            DspNode::Master => {}
            DspNode::Volume(ref mut volume) => volume.audio_requested(buffer, sample_hz),
            DspNode::Player(ref mut player) => player.audio_requested(buffer, sample_hz),
        }
    }
}

pub fn new_player() -> DspNode {
    DspNode::Player(PlayerNode::new())
}

pub fn new_volume() -> DspNode {
    DspNode::Volume(VolumeNode::new())
}
