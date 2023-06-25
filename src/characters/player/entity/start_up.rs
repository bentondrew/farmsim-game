use bevy::prelude::{
    default, shape, Assets, BuildChildren, Bundle, Color, Commands, Mesh, PbrBundle, ResMut,
    StandardMaterial, Transform, Vec3,
};

use super::super::super::components::Name;
use super::super::camera::start_up::{create_camera_3d_bundle, create_player_camera_component};
use super::components::PlayerCharacter;

/// A component bundle used to initialize a player character.
#[derive(Bundle)]
struct PlayerInitBundle {
    character_type: PlayerCharacter,
    name: Name,
    renderer_representation: PbrBundle,
}

/// Creates the bundle to add the player entity.
fn create_player_init_bundle(
    player_id: u8,
    spawn_location: Vec3,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> PlayerInitBundle {
    let player_height_mid_point = 1.0;
    let initial_player_translation = spawn_location + Vec3::new(0.0, player_height_mid_point, 0.0);
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
    return bundle;
}

/// Generates a system that adds a player and their camera with the id provided.
/// The system also takes in a location at which to spawn the player and associated
/// camera.
pub fn generate_add_player_system(
    player_id: u8,
    spawn_location: Vec3,
) -> impl Fn(Commands, ResMut<Assets<Mesh>>, ResMut<Assets<StandardMaterial>>) {
    move |mut commands: Commands,
          mut meshes: ResMut<Assets<Mesh>>,
          mut materials: ResMut<Assets<StandardMaterial>>| {
        let player_init_bundle =
            create_player_init_bundle(player_id, spawn_location, &mut meshes, &mut materials);
        let player_entity = commands.spawn(player_init_bundle).id();
        let player_camera_component = create_player_camera_component(player_id);
        let mut camera_entity_commands = commands.spawn(player_camera_component);
        camera_entity_commands.set_parent(player_entity);
        let camera_3d_bundle = create_camera_3d_bundle();
        camera_entity_commands.insert(camera_3d_bundle);
    }
}
