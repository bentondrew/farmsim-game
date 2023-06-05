use bevy::prelude::{
    default, shape, Assets, Color, Commands, Mesh, PbrBundle, ResMut, StandardMaterial,
};

/// Initial system to render a flat ground plane.
pub fn add_ground_plane(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(50.0).into()),
        material: materials.add(Color::SILVER.into()),
        ..default()
    });
}
