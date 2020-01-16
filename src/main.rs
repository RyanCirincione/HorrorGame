// Example 1: The Square
// Open a window, and draw a colored square in it
use hecs::*;
use quicksilver::{
    geom::{Circle, Rectangle, Vector},
    graphics::{Color, Graphics},
    lifecycle::{run, ElementState, Event::*, EventStream, MouseButton, Settings, Window},
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
    let player_id = spawn_player(&mut world);
    spawn_tree(&mut world, Vector::new(500.0, 200.0));
    let mut mouse_pos = Vector::ZERO;
    loop {
        while let Some(ev) = events.next_event().await {
            // input
            match ev {
                MouseMoved { position, .. } => {
                    mouse_pos = position.into();
                }
                MouseInput { state: ElementState::Released, button: MouseButton::Left, .. } => {
                    let player_bounds = *world.get::<Circle>(player_id).unwrap();
                    spawn_hook(&mut world, player_bounds.pos, (mouse_pos - player_bounds.pos).normalize() * 10);
                },
                _ => ()
            }
        }
        // update
        for (_id, circle) in world.query::<&mut Circle>().iter() {
            circle.pos += Vector::ONE;
        }
        for (_id, (circle, vel, age)) in world.query::<(&mut Circle, &mut Vector, &mut u32)>().iter() {
            println!("{}", vel);
            circle.pos += *vel;
            *age += 1;
            if *age == 30 {
                *vel = *vel * -1;
            } else if *age > 60 {
                //TODO delete
            }
        }
        // draw
        gfx.clear(Color::WHITE);
        // draw shadow map
        // draw the world
        gfx.fill_circle(mouse_pos, 10.0, Color::GREEN);
        let player_bounds = world.get::<Circle>(player_id).unwrap();
        gfx.fill_circle(player_bounds.pos, player_bounds.radius, Color::BLUE);
        for (_id, (circle, _tree)) in world.query::<(&Circle, &Tree)>().iter() {
            gfx.fill_circle(circle.pos, circle.radius, Color::RED);
        }
        for (_id, (circle, _vel)) in world.query::<(&Circle, &Vector)>().iter() {
            gfx.fill_circle(circle.pos, circle.radius, Color::YELLOW);
        }
        gfx.present(&window)?;
    }
}

fn spawn_hook(world: &mut World, pos: Vector, vel: Vector) -> Entity {
    let bounds = Circle::new(pos, 4.0);
    let age: u32 = 0;

    world.spawn((bounds, vel, age))
}

fn spawn_player(world: &mut World) -> Entity {
    let bounds = Circle::new(Vector::new(0.0, 0.0), 16.0);
    let light = LightSource { start_angle: 0.0, end_angle: 30.0, radius: 5.0 };

    world.spawn((bounds, light))
}

fn spawn_tree(world: &mut World, pos: Vector) -> Entity {
    let bounds = Circle::new(pos, 12.0);

    world.spawn((bounds, AnchorPoint, Tree))
}

struct LightSource {
    start_angle: f32,
    end_angle: f32,
    radius: f32
}

struct AnchorPoint;

struct Tree;
