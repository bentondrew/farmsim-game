use bevy::prelude::{default, Camera3dBundle, Commands, Transform, Vec3};

pub fn add_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-25., 6., 0.).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
        ..default()
    });
}
