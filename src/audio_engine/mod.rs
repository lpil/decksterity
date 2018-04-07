mod dsp_node;

use dsp;
use dsp::Graph;
use self::dsp_node::DspNode;
use super::media;

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
    let master_vol = graph.add_node(dsp_node::new_volume());
    graph.set_master(Some(master));

    graph
        .add_connection(master_vol, master)
        .expect("graph feedback loop");

    let (_, deck_graph_index) = graph.add_input(dsp_node::new_player(), master_vol);

    AudioEngine {
        graph,
        deck_graph_index,
        master_vol_index: master_vol,
    }
}

impl AudioEngine {
    pub fn set_volume(&mut self, amount: f32) -> f32 {
        self.master_volume_mut().set_volume(amount)
    }

    pub fn set_media(&mut self, media: media::Media) {
        self.deck_mut().set_media(media)
    }

    pub fn toggle_play_pause(&mut self) -> bool {
        self.deck_mut().toggle_play_pause()
    }

    pub fn adjust_pitch(&mut self, delta: dsp_node::Pitch) -> dsp_node::Pitch {
        self.deck_mut().adjust_pitch(delta)
    }

    // pub fn set_pitch(&mut self, new_pitch: dsp_node::Pitch) -> dsp_node::Pitch {
    //     self.deck_mut().set_pitch(new_pitch)
    // }

    fn deck_mut(&mut self) -> &mut dsp_node::Player {
        let node = self.graph
            .node_mut(self.deck_graph_index)
            .expect("Deck not found");
        match node {
            &mut DspNode::Player(ref mut buffer_player) => buffer_player,
            ref other => panic!("Expected Player, got {:?}", other),
        }
    }

    fn master_volume_mut(&mut self) -> &mut dsp_node::Volume {
        let node = self.graph
            .node_mut(self.master_vol_index)
            .expect("Master Volume not found");
        match node {
            &mut DspNode::Volume(ref mut volume) => volume,
            ref other => panic!("Expected volume, got {:?}", other),
        }
    }
}
