pub mod dsp_node;

extern crate cpal;
extern crate dsp;

use std::mem;
use super::media;
use self::dsp::{Graph, Node};
use self::dsp::sample::ToFrameSliceMut;
use self::dsp_node::DspNode;

pub const CHANNELS: usize = 2;
pub const SAMPLE_HZ: f64 = 44_100.0;

pub type Sample = f32;
pub type Frame = [Sample; CHANNELS];

pub struct AudioEngine {
    pub graph: dsp::Graph<[Sample; CHANNELS], DspNode>,
    deck_graph_index: dsp::NodeIndex,
}

pub fn new() -> AudioEngine {
    let mut graph = Graph::new();

    let master = graph.add_node(DspNode::Master);
    let master_vol = graph.add_node(DspNode::Volume(1.0));
    graph.set_master(Some(master));

    graph
        .add_connection(master_vol, master)
        .expect("feedback loop");

    let (_, deck_graph_index) = graph.add_input(DspNode::Player(0.0, 1.0, vec![]), master_vol);

    AudioEngine {
        graph,
        deck_graph_index,
    }
}

impl AudioEngine {
    pub fn set_media(&mut self, media: media::Media) {
        match self.deck_mut() {
            &mut DspNode::Player(ref mut phase, _pitch, ref mut samples) => {
                mem::replace(samples, media);
                *phase = 0.0;
            }
            ref other => panic!("Expected Player, got {:?}", other),
        }
    }

    fn deck_mut(&mut self) -> &mut DspNode {
        self.graph
            .node_mut(self.deck_graph_index)
            .expect("Deck not found")
    }

    pub fn set_pitch(&mut self, new_pitch: dsp_node::Pitch) {
        match self.deck_mut() {
            &mut DspNode::Player(_, ref mut pitch, _) => {
                *pitch = new_pitch;
            }
            ref other => panic!("Expected Player, got {:?}", other),
        }
    }

    pub fn connect_to_output(&mut self) {}
}
