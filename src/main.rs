// Example 1: The Square
// Open a window, and draw a colored square in it
use hecs::*;
use quicksilver::{
    geom::{Circle, Rectangle, Vector},
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
    let mut world = World::new();
    let player_id = world.spawn((Player, Circle::new(Vector::new(0.0, 0.0), 16.0)));
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
        for (_id, circle) in world.query::<&mut Circle>().iter() {
            circle.pos += Vector::ONE;
        }
        // draw
        gfx.clear(Color::WHITE);
        gfx.fill_circle(mouse_pos, 10.0, Color::GREEN);
        let player_bounds = world.get::<Circle>(player_id).unwrap();
        gfx.fill_circle(player_bounds.pos, player_bounds.radius, Color::BLUE);
        gfx.present(&window)?;
    }
}

struct Player;
