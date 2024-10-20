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

/**
 * Systems: normal Rust functions
 */
fn print_position_system(query: Query<&Position>) {
    for position in &query {
        println!("position: {} {}", position.x, position.y);
    }
}

// Entities: a simple type containing a unique integer
struct Entity(u64);
