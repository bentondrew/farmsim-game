mod cameras;
mod characters;
mod game_world;
mod lighting;
mod player_control;

use bevy::{
    app::App,
    prelude::{default, DefaultPlugins, PluginGroup},
    window::{Window, WindowPlugin},
};
use cameras::generate_add_camera_system;
use characters::players::generate_add_player_system;
use game_world::add_ground_plane;
use lighting::add_light;
use player_control::{gamepad_connection_events, generate_move_player_system};

/// Creates and runs the game application based on the bevy engine crate.
fn main() {
    let player_id = 0;
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Farmsim Game".into(),
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(generate_add_player_system(player_id))
        .add_startup_system(add_ground_plane)
        .add_startup_system(add_light)
        .add_startup_system(generate_add_camera_system(player_id))
        .add_system(gamepad_connection_events)
        .add_system(generate_move_player_system(player_id))
        .run();
}
