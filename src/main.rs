// Example 1: The Square
// Open a window, and draw a colored square in it
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color, Graphics},
    lifecycle::{run, ElementState, Event::*, EventStream, Settings, Window},
    Result,
};

fn main() {
    run(
        Settings {
            size: Vector::new(800.0, 600.0).into(),
            title: "Square Example",
            ..Settings::default()
        },
        app,
    );
}

async fn app(window: Window, mut gfx: Graphics, mut events: EventStream) -> Result<()> {
    let mut mouse_pos = Vector::ZERO;
    loop {
        while let Some(ev) = events.next_event().await {
            // input
            match ev {
                MouseMoved { position, .. } => {
                    mouse_pos = position.into();
                }
                MouseInput { state: ElementState::Released, button, .. } => {
                    // TODO handle mice
                },
                _ => ()
            }
        }
        // update
        // draw
        gfx.clear(Color::WHITE);
        gfx.fill_circle(mouse_pos, 10.0, Color::GREEN);
        gfx.present(&window)?;
    }
}
