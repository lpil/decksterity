mod dsp_node;

use std::mem;
use super::media;
use dsp;
use dsp::Graph;
use self::dsp_node::DspNode;

pub const CHANNELS: usize = 2;

pub type Sample = f32;
pub type Frame = [Sample; CHANNELS];

pub struct AudioEngine {
    pub graph: dsp::Graph<[Sample; CHANNELS], DspNode>,
    deck_graph_index: dsp::NodeIndex,
    master_vol_index: dsp::NodeIndex,
}

pub fn new() -> AudioEngine {
    let mut graph = Graph::new();

    let master = graph.add_node(DspNode::Master);
    let master_vol = graph.add_node(DspNode::Volume(0.0));
    graph.set_master(Some(master));

    graph
        .add_connection(master_vol, master)
        .expect("feedback loop");

    let (_, deck_graph_index) = graph.add_input(dsp_node::new_player(), master_vol);

    AudioEngine {
        graph,
        deck_graph_index,
        master_vol_index: master_vol,
    }
}

impl AudioEngine {
    pub fn set_volume(&mut self, volume: dsp_node::Amp) -> dsp_node::Amp {
        match self.master_volume_mut() {
            &mut DspNode::Volume(ref mut v) => {
                mem::replace(v, volume);
                volume
            }
            ref other => panic!("Expected Player, got {:?}", other),
        }
    }

    pub fn set_media(&mut self, media: media::Media) {
        match self.deck_mut() {
            &mut DspNode::Player(_, ref mut phase, _, ref mut samples) => {
                mem::replace(samples, media);
                *phase = 0.0;
            }
            ref other => panic!("Expected Player, got {:?}", other),
        }
    }

    pub fn toggle_play_pause(&mut self) -> dsp_node::IsPlaying {
        match self.deck_mut() {
            &mut DspNode::Player(ref mut is_playing, _, _, _) => {
                let new_value = !*is_playing;
                *is_playing = new_value;
                new_value
            }
            ref other => panic!("Expected Player, got {:?}", other),
        }
    }

    pub fn adjust_pitch(&mut self, delta: dsp_node::Pitch) -> dsp_node::Pitch {
        match self.deck_mut() {
            &mut DspNode::Player(_, _, ref mut pitch, _) => {
                let new_pitch = *pitch + delta;
                *pitch = new_pitch;
                new_pitch
            }
            ref other => panic!("Expected Player, got {:?}", other),
        }
    }

    pub fn set_pitch(&mut self, new_pitch: dsp_node::Pitch) -> dsp_node::Pitch {
        match self.deck_mut() {
            &mut DspNode::Player(_, _, ref mut pitch, _) => {
                *pitch = new_pitch;
                new_pitch
            }
            ref other => panic!("Expected Player, got {:?}", other),
        }
    }

    fn deck_mut(&mut self) -> &mut DspNode {
        self.graph
            .node_mut(self.deck_graph_index)
            .expect("Deck not found")
    }

    fn master_volume_mut(&mut self) -> &mut DspNode {
        self.graph
            .node_mut(self.master_vol_index)
            .expect("Master Volume not found")
    }
}
