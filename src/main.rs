use bevy::{app::App, prelude::{Component, Commands, Query, With}};

fn hello_world() {
    println!("Hello world!")
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Rachel".to_string())));
    commands.spawn((Person, Name("Benton".to_string())));
    commands.spawn((Person, Name("Ryan".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("Hello {}!", name.0);
    }
}

fn main() {
    App::new().add_startup_system(add_people).add_system(hello_world).add_system(greet_people).run();
}
