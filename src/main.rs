use bevy::{app::App, prelude::{Component, Commands, ResMut, Assets, Mesh, StandardMaterial, Bundle, PbrBundle, shape, Color, default, PointLightBundle, PointLight, Transform, Camera3dBundle, Vec3, DefaultPlugins, PluginGroup}, window::{WindowPlugin, Window}};

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Bundle)]
struct PlayerInitBundle {
    being: Person,
    name: Name,
    renderer_representation: PbrBundle,
}

fn add_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let bundle = PlayerInitBundle {
        being: Person,
        name: Name("Player1".to_string()),
        renderer_representation: PbrBundle {
            mesh: meshes.add(
                Mesh::try_from(shape::Icosphere {
                    radius: 1.00,
                    subdivisions: 32,
                })
                .unwrap(),
            ),
            material: materials.add(
                StandardMaterial {
                    base_color: Color::hex("#71daff").unwrap(),
                    ..default()
                }
            ),
            transform: Transform::from_xyz(0., 1., 0.),
            ..default()
        }
    };
    commands.spawn(bundle);
}

fn add_light(
    mut commands: Commands,
) {
    commands.spawn(
        PointLightBundle {
            point_light: PointLight {
                intensity: 600000.,
                range: 100.,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(50., 50., 50.0),
            ..default()
        }
    );
}

fn add_ground_plane(
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

fn add_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle{
        transform: Transform::from_xyz(0., 6., 12.).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
        ..default()
    });
}

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
    .run();
}
