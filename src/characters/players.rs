use bevy::prelude::{
    default, shape, Assets, Bundle, Color, Commands, Component, Mesh, PbrBundle, ResMut,
    StandardMaterial, Transform,
};

use super::common::Name;

/// A component to indicate if an entity is a player character.
#[derive(Component)]
pub struct PlayerCharacter {
    id: u8,
}

/// A component bundle used to initialize a player character.
#[derive(Bundle)]
struct PlayerInitBundle {
    character_type: PlayerCharacter,
    name: Name,
    renderer_representation: PbrBundle,
}

/// An initial system to spawn a player character entity.
pub fn add_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let bundle = PlayerInitBundle {
        character_type: PlayerCharacter { id: 0 },
        name: Name("Player1".to_string()),
        renderer_representation: PbrBundle {
            mesh: meshes.add(
                Mesh::try_from(shape::Icosphere {
                    radius: 1.00,
                    subdivisions: 32,
                })
                .unwrap(),
            ),
            material: materials.add(StandardMaterial {
                base_color: Color::hex("#71daff").unwrap(),
                ..default()
            }),
            transform: Transform::from_xyz(0., 1., 0.),
            ..default()
        },
    };
    commands.spawn(bundle);
}
