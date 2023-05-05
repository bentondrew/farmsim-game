use bevy::{
    app::App,
    prelude::{
        Component,
        Commands,
        ResMut,
        Assets,
        Mesh,
        StandardMaterial,
        Bundle,
        PbrBundle,
        shape,
        Color,
        default,
        PointLightBundle,
        PointLight,
        Transform,
        Camera3dBundle,
        Vec3,
        DefaultPlugins,
        PluginGroup, 
        EventReader,
        info,
        Query,
        Entity,
        With,
        Without
    },
    window::{WindowPlugin, Window},
    input::gamepad::{
        GamepadConnectionEvent,
        // GamepadAxisChangedEvent,
        // GamepadButtonChangedEvent
    },
    input::gamepad::{GamepadConnection::{Connected, Disconnected}, GamepadInfo}
};

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct PlayerCharacter;

#[derive(Component)]
struct Controller {
    gamepad_id: usize,
}

// #[derive(Component)]
// struct NonPlayerCharacter;

#[derive(Bundle)]
struct PlayerInitBundle {
    being: Person,
    character_type: PlayerCharacter,
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
        character_type: PlayerCharacter,
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
            transform: Transform::from_xyz(50., 50., 50.),
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
        transform: Transform::from_xyz(-25., 6., 0.).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
        ..default()
    });
}

fn connect_controller_to_player(
    commands: &mut Commands,
    connection_event: &GamepadConnectionEvent,
    gamepad_info: &GamepadInfo,
    players_without_controllers: &Query<(Entity, &Name), (With<PlayerCharacter>, Without<Controller>)>,
) {
    // For the first player entity in the query, add the connected controller.
    for (player_entity, player_name) in players_without_controllers {
        commands.entity(player_entity).insert(
            Controller{gamepad_id: connection_event.gamepad.id}
        );
        info!(
            "Gamepad {} of id {} assigned to player {}",
            gamepad_info.name, connection_event.gamepad.id, player_name.0
        );
        break;
    }
}

fn disconnect_controller_from_player(
    commands: &mut Commands,
    connection_event: &GamepadConnectionEvent,
    players_with_controllers: &Query<(Entity, &Name, &Controller), (With<PlayerCharacter>, With<Controller>)>,
) {
    // Find the first entity with a controlled with a gamepad id
    // matching the gamepad id from the event, then remove the
    // controller from that entity.
    for (player_entity, player_name, controller) in players_with_controllers {
        if controller.gamepad_id == connection_event.gamepad.id {
            commands.entity(player_entity).remove::<Controller>();
            info!(
                "Controller with gamepad of id {} removed from player {}",
                connection_event.gamepad.id, player_name.0
            );
        }
        break;
    }
}

fn gamepad_connection_events (
    mut commands: Commands,
    mut connection_events: EventReader<GamepadConnectionEvent>,
    players_with_controllers: Query<(Entity, &Name, &Controller), (With<PlayerCharacter>, With<Controller>)>,
    players_without_controllers: Query<(Entity, &Name), (With<PlayerCharacter>, Without<Controller>)>,
) {
    for connection_event in connection_events.iter() {
        match &connection_event.connection {
            Connected(gamepad_info) => connect_controller_to_player(
                    &mut commands,
                    connection_event,
                    gamepad_info,
                    &players_without_controllers,
                ),
            Disconnected => disconnect_controller_from_player(
                    &mut commands,
                    connection_event, 
                    &players_with_controllers
                ),
        }
    }
}

// fn gamepad_events(
//     mut connection_events: EventReader<GamepadConnectionEvent>,
//     // mut axis_events: EventReader<GamepadAxisChangedEvent>,
//     // mut button_events: EventReader<GamepadButtonChangedEvent>,
// ) {
//     for connection_event in connection_events.iter() {
//         info!("{:?}", connection_event);
//     }
//     // for axis_event in axis_events.iter() {
//     //     info!(
//     //         "{:?} of {:?} is changed to {}",
//     //         axis_event.axis_type, axis_event.gamepad, axis_event.value
//     //     );
//     // }
//     // for button_event in button_events.iter() {
//     //     info!(
//     //         "{:?} of {:?} is changed to {}",
//     //         button_event.button_type, button_event.gamepad, button_event.value
//     //     );
//     // }
// }

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
    .add_system(gamepad_connection_events)
    .run();
}
