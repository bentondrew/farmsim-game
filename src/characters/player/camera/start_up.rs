use bevy::prelude::{default, Bundle, Camera3dBundle, Transform, Vec3};

use super::components::PlayerCamera;

/// A component bundle used to initialize a player camera.
#[derive(Bundle)]
pub struct PlayerCameraInitBundle {
    player_camera: PlayerCamera,
    camera_3d_bundle: Camera3dBundle,
}

/// Creates a component bundle for spawning a camera for a player.
pub fn create_camera_init_bundle(
    player_id: u8,
    player_spawn_location: Vec3,
) -> PlayerCameraInitBundle {
    let initial_camera_translation = player_spawn_location + Vec3::new(-25.0, 6.0, 0.0);
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
    return bundle;
}
