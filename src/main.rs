use bevy::prelude::*;

fn main() {
    App::new().run();
}

/**
 * Components: Rust structs that implement the Component trait
 */
#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}
