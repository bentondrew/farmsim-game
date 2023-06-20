use bevy::prelude::{
    default, shape, Assets, Bundle, Color, Commands, Component, Mesh, PbrBundle, ResMut,
    StandardMaterial, Transform, Vec3,
};

use super::common::Name;

/// A component to indicate if an entity is a player character.
#[derive(Component)]
pub struct PlayerCharacter {
    pub id: u8,
    pub player_height_mid_point: f32,
}

/// A component bundle used to initialize a player character.
#[derive(Bundle)]
struct PlayerInitBundle {
    character_type: PlayerCharacter,
    name: Name,
    renderer_representation: PbrBundle,
}

/// Generates a system that adds a player with the id provided. The system also takes
/// in a location at which to spawn the player.
pub fn generate_add_player_system(
    player_id: u8,
    spawn_location: Vec3,
) -> impl Fn(Commands, ResMut<Assets<Mesh>>, ResMut<Assets<StandardMaterial>>) {
    move |mut commands: Commands,
          mut meshes: ResMut<Assets<Mesh>>,
          mut materials: ResMut<Assets<StandardMaterial>>| {
        let player_height_mid_point = 1.0;
        let initial_player_translation =
            spawn_location + Vec3::new(0.0, player_height_mid_point, 0.0);
        let bundle = PlayerInitBundle {
            character_type: PlayerCharacter {
                id: player_id,
                player_height_mid_point: player_height_mid_point,
            },
            name: Name("Player1".to_string()),
            renderer_representation: PbrBundle {
                mesh: meshes.add(
                    Mesh::try_from(shape::Icosphere {
                        radius: player_height_mid_point,
                        subdivisions: 32,
                    })
                    .unwrap(),
                ),
                material: materials.add(StandardMaterial {
                    base_color: Color::hex("#71daff").unwrap(),
                    ..default()
                }),
                transform: Transform::from_translation(initial_player_translation),
                ..default()
            },
        };
        commands.spawn(bundle);
    }
}
