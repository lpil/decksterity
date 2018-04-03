use conrod::{widget, Colorable, Positionable, Widget};
use super::constants::*;

#[derive(WidgetCommon)]
pub struct Deck {
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    style: Style,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, WidgetStyle)]
pub struct Style {}

widget_ids! {
    struct Ids {
        master,
        info_section,
        title,
        artist,
        waveform_section,
    }
}

/// Represents the unique, cached state for our Deck widget.
pub struct State {
    ids: Ids,
}

impl Deck {
    /// Create a button context to be built upon.
    pub fn new() -> Self {
        Deck {
            common: widget::CommonBuilder::default(),
            style: Style::default(),
        }
    }
}

impl Widget for Deck {
    type State = State;
    type Style = Style;
    type Event = ();

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
        State {
            ids: Ids::new(id_gen),
        }
    }

    fn style(&self) -> Self::Style {
        self.style
    }

    /// Update the state of the button by handling any input that has occurred since the last
    /// update.
    fn update(self, args: widget::UpdateArgs<Self>) -> Self::Event {
        let widget::UpdateArgs { id, state, ui, .. } = args;

        widget::Canvas::new()
            .length(DECK_HEIGHT)
            .mid_top_of(id)
            .flow_right(&[
                (
                    state.ids.info_section,
                    widget::Canvas::new().length(DECK_INFO_WIDTH).pad_left(PAD),
                ),
                (
                    state.ids.waveform_section,
                    widget::Canvas::new().color(WAVEFORM_COLOUR),
                ),
            ])
            .graphics_for(id)
            .set(state.ids.master, ui);

        // info_section

        widget::Text::new("Nero's Day At Disneyland")
            .color(TEXT_COLOUR)
            .font_size(FONT_SIZE)
            .top_left_of(state.ids.info_section)
            .graphics_for(id)
            .set(state.ids.artist, ui);

        widget::Text::new("No Money Down Low Monthly Payments")
            .color(TEXT_COLOUR)
            .font_size(FONT_SIZE)
            .top_left_of(state.ids.info_section)
            .down(LINE_PAD)
            .graphics_for(id)
            .set(state.ids.title, ui);
    }
}
