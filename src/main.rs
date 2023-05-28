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
use cameras::add_camera;
use characters::players::add_player;
use game_world::add_ground_plane;
use lighting::add_light;
use player_control::{gamepad_axis_changed_events, gamepad_connection_events};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Farmsim Game".into(),
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(add_player)
        .add_startup_system(add_ground_plane)
        .add_startup_system(add_light)
        .add_startup_system(add_camera)
        .add_system(gamepad_connection_events)
        .add_system(gamepad_axis_changed_events)
        .run();
}
