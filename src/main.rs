use bevy::prelude::*;

fn main() {
    App::new()
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world, greet_people))
        .run();
}

// System
fn hello_world() {
    println!("hello world!");
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
    commands.spawn((Person, Name("Đỗ Quyên".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        if(name.0 == "Đỗ Quyên") {
            println!("Ich liebe Dich, {}", name.0);
        } else {
            println!("hello {}!", name.0);
        }
    }
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
