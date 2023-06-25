use bevy::prelude::{default, Camera3dBundle, Transform, Vec3};

use super::components::PlayerCamera;

// Create a player camera component.
pub fn create_player_camera_component(player_id: u8) -> PlayerCamera {
    return PlayerCamera {
        player_id: player_id,
    };
}

/// Creates a camera 3d bundle.
pub fn create_camera_3d_bundle() -> Camera3dBundle {
    let look_location = Vec3::new(0.0, 0.0, 0.0);
    let initial_camera_translation = Vec3::new(-25.0, 6.0, 0.0);
    return Camera3dBundle {
        transform: Transform::from_translation(initial_camera_translation)
            .looking_at(look_location, Vec3::Y),
        ..default()
    };
}
