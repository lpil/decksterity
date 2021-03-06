use midir::{Ignore, MidiInput, MidiInputConnection};
use std::sync::{Arc, Mutex};
use super::engine::Engine;

type Ctrl = (u8, u8);

const PITCH_STEP: f64 = 0.00_02;
const TOP_ENCODER_4_TURN: Ctrl = (190, 3);
const TOP_ENCODER_4_PRESS: Ctrl = (158, 55);
const SLIDER_4: Ctrl = (190, 19);

pub fn connect_xone_k2(engine: Arc<Mutex<Engine>>) -> Option<MidiInputConnection<()>> {
    // Set up MIDI input
    let mut midi_in = MidiInput::new("xone:k2").expect("Unable to create MIDI input");
    midi_in.ignore(Ignore::All);

    let in_port = (0..midi_in.port_count())
        .find(|i| midi_in.port_name(*i) == Ok("XONE:K2 20:0".to_string()))?;

    let handle_msg = move |msg: &[u8]| {
        assert_eq!(msg.len(), 3);
        match ((msg[0], msg[1]), msg[2]) {
            (TOP_ENCODER_4_TURN, 1) => {
                let new_pitch = engine.lock().unwrap().adjust_pitch(PITCH_STEP);
                println!("Pitch: {:?}", new_pitch);
            }
            (TOP_ENCODER_4_TURN, 127) => {
                let new_pitch = engine.lock().unwrap().adjust_pitch(-PITCH_STEP);
                println!("Pitch: {:?}", new_pitch);
            }
            (TOP_ENCODER_4_PRESS, 127) => {
                let is_playing = engine.lock().unwrap().toggle_play_pause();
                println!("Playing: {:?}", is_playing);
            }
            (SLIDER_4, velocity) => {
                let volume = f32::from(velocity) / 127.0;
                engine.lock().unwrap().set_volume(volume);
                println!("Volume: {:?}", volume);
            }
            _ => println!("{:?}", msg),
        }
    };

    // _conn_in needs to be a named parameter because it needs to be kept
    // alive until the end of the scope
    let input = midi_in
        .connect(in_port, "xone:k2-in", move |_, msg, _| handle_msg(msg), ())
        .expect("Unable to connect to XONE:K2 input");
    Some(input)
}
