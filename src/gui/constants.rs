#![allow(dead_code)]

use conrod::color;
use conrod::color::Color;

const WHITE: Color = Color::Rgba(1.0, 1.0, 1.0, 1.0);
const BLACK: Color = Color::Rgba(0.0, 0.0, 0.0, 1.0);
const GREY: Color = Color::Rgba(50.0 / 255.0, 50.0 / 255.0, 50.0 / 255.0, 1.0);
const PINK: Color = Color::Rgba(1.0, 20.0 / 255.0, 147.0 / 255.0, 1.0);
const TRANSPARENT: Color = color::TRANSPARENT;

pub const TEXT_COLOUR: Color = WHITE;
pub const BORDER_COLOUR: Color = GREY;
pub const WAVEFORM_COLOUR: Color = PINK;

pub const PAD: f64 = 6.0;

pub const DECK_HEIGHT: f64 = 150.0;
pub const DECK_INFO_WIDTH: f64 = 300.0;

pub const FONT_SIZE: u32 = 15;
pub const LINE_PAD: f64 = 2.0;
