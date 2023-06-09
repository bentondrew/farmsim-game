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

/// Generates a system that adds a camera associated with the player id provided.
pub fn generate_add_camera_system(player_id: u8) -> impl Fn(Commands) {
    move |mut commands: Commands| {
        let bundle = PlayerCameraInitBundle {
            player_camera: PlayerCamera {
                player_id: player_id,
            },
            camera_3d_bundle: Camera3dBundle {
                transform: Transform::from_xyz(-25., 6., 0.)
                    .looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
                ..default()
            },
        };
        commands.spawn(bundle);
    }
}
