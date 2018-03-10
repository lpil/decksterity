mod deck;
mod constants;

use std;
use conrod;
use conrod::backend::glium::glium;
use conrod::backend::glium::glium::Surface;
use conrod::text::FontCollection;
use self::constants::*;

const NOTO_SANS_REGULAR: &'static [u8] =
    include_bytes!("../../assets/fonts/NotoSans/NotoSans-Regular.ttf");

const INITIAL_WIDTH: u32 = 800;
const INITIAL_HEIGHT: u32 = 600;

widget_ids! {
    struct Ids {
        master,
        library,
        library_canvas,
        deck_a,
        deck_a_canvas,
        deck_b,
        deck_b_canvas,
    }
}

pub fn start() {
    // Build the window.
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title("Decksterity")
        .with_dimensions(INITIAL_WIDTH, INITIAL_HEIGHT);
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    // construct our `Ui`.
    let mut ui = conrod::UiBuilder::new([INITIAL_WIDTH as f64, INITIAL_HEIGHT as f64]).build();

    let font = FontCollection::from_bytes(NOTO_SANS_REGULAR)
        .into_font()
        .expect("Failed to parse font");
    ui.fonts.insert(font);

    // A type used for converting `conrod::render::Primitives` into `Command`s that can be used
    // for drawing to the glium `Surface`.
    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

    // The image map describing each of our widget->image mappings (in our case, none).
    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

    // Instantiate the generated list of widget identifiers.
    let ids = &mut Ids::new(ui.widget_id_generator());

    // Poll events from the window.
    let mut event_loop = EventLoop::new();
    'main: loop {
        // Handle all events.
        for event in event_loop.next(&mut events_loop) {
            // Use the `winit` backend feature to convert the winit event to a conrod one.
            if let Some(event) = conrod::backend::winit::convert_event(event.clone(), &display) {
                ui.handle_event(event);
                event_loop.needs_update();
            }

            match event {
                glium::glutin::Event::WindowEvent { event, .. } => match event {
                    // TODO: Ask for confirmation here. Or don't close if deck playing.
                    // Break from the loop when window closed.
                    glium::glutin::WindowEvent::Closed => break 'main,
                    _ => (),
                },
                _ => (),
            }
        }

        // Instantiate all widgets in the GUI.
        set_widgets(ui.set_widgets(), ids);

        // Render the `Ui` and then display it on the screen.
        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }
    }
}

// Draw the Ui.
fn set_widgets(ref mut ui: conrod::UiCell, ids: &mut Ids) {
    use conrod::{widget, Positionable, Widget};

    widget::Canvas::new()
        .flow_down(&[
            (ids.deck_a_canvas, widget::Canvas::new().length(DECK_HEIGHT)),
            (ids.deck_b_canvas, widget::Canvas::new().length(DECK_HEIGHT)),
            (ids.library_canvas, widget::Canvas::new()),
        ])
        .set(ids.master, ui);

    deck::Deck::new()
        .mid_top_of(ids.deck_a_canvas)
        .set(ids.deck_a, ui);

    deck::Deck::new()
        .mid_top_of(ids.deck_b_canvas)
        .set(ids.deck_b, ui);

    widget::Canvas::new()
        .mid_top_of(ids.library_canvas)
        .set(ids.library, ui);
}

/// This `Iterator`-like type simplifies some of the boilerplate involved in setting up a
/// glutin+glium event loop that works efficiently with conrod.
pub struct EventLoop {
    ui_needs_update: bool,
    last_update: std::time::Instant,
}

impl EventLoop {
    pub fn new() -> Self {
        EventLoop {
            last_update: std::time::Instant::now(),
            ui_needs_update: true,
        }
    }

    /// Produce an iterator yielding all available events.
    pub fn next(
        &mut self,
        events_loop: &mut glium::glutin::EventsLoop,
    ) -> Vec<glium::glutin::Event> {
        // We don't want to loop any faster than 60 FPS, so wait until it has been at least 16ms
        // since the last yield.
        let last_update = self.last_update;
        let sixteen_ms = std::time::Duration::from_millis(16);
        let duration_since_last_update = std::time::Instant::now().duration_since(last_update);
        if duration_since_last_update < sixteen_ms {
            std::thread::sleep(sixteen_ms - duration_since_last_update);
        }

        // Collect all pending events.
        let mut events = Vec::new();
        events_loop.poll_events(|event| events.push(event));

        // If there are no events and the `Ui` does not need updating, wait for the next event.
        if events.is_empty() && !self.ui_needs_update {
            events_loop.run_forever(|event| {
                events.push(event);
                glium::glutin::ControlFlow::Break
            });
        }

        self.ui_needs_update = false;
        self.last_update = std::time::Instant::now();

        events
    }

    /// Notifies the event loop that the `Ui` requires another update whether or not there are any
    /// pending events.
    ///
    /// This is primarily used on the occasion that some part of the `Ui` is still animating and
    /// requires further updates to do so.
    pub fn needs_update(&mut self) {
        self.ui_needs_update = true;
    }
}
