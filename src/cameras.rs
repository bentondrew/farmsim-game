use bevy::prelude::{default, Bundle, Camera3dBundle, Commands, Component, Transform, Vec3};

/// A component to put on a camera entity to associate it with a player.
#[derive(Component)]
pub struct PlayerCamera {
    pub player_id: u8,
}

/// A component bundle used to initialize a player camera.
#[derive(Bundle)]
pub struct PlayerCameraInitBundle {
    player_camera: PlayerCamera,
    camera_3d_bundle: Camera3dBundle,
}

/// Generates a system that adds a camera associated with the player id provided. The
/// system spawns the camera relative to the provided player_spawn_location and looking
/// at it. This assumes that the provided location is the player location. This is so
/// the system can run in parallel to the player spawning system.
///
/// An alternative is to query the player characters and spawn relative to the player's
/// transform. That would mean this system would have to run after the player spawn
/// system and would require a system dependency in the app definition of systems.
pub fn generate_add_camera_system(player_id: u8, player_spawn_location: Vec3) -> impl Fn(Commands) {
    let initial_camera_translation = player_spawn_location + Vec3::new(-25.0, 6.0, 0.0);
    move |mut commands: Commands| {
        let bundle = PlayerCameraInitBundle {
            player_camera: PlayerCamera {
                player_id: player_id,
            },
            camera_3d_bundle: Camera3dBundle {
                transform: Transform::from_translation(initial_camera_translation)
                    .looking_at(player_spawn_location, Vec3::Y),
                ..default()
            },
        };
        commands.spawn(bundle);
    }
}
