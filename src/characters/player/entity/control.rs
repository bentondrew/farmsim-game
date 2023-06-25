use bevy::{
    input::gamepad::{
        GamepadConnection::{Connected, Disconnected},
        GamepadConnectionEvent, GamepadInfo,
    },
    prelude::{
        info, Axis, Commands, Component, Entity, EventReader, Gamepad, GamepadAxis,
        GamepadAxisType, Gamepads, Query, Res, Time, Transform, Vec3, Without,
    },
};

use super::components::PlayerCharacter;

/// A Bevy Engine component that is attached to an entity that represents the resource
/// that controls that entity. Expected to be attached to entities that also have the
/// Player component. Currently only supports Gamepad controls.
#[derive(Component)]
pub struct Controller {
    gamepad: Gamepad,
}

/// Contains player information for determining player movement.
struct PlayerInfo {
    entity: Entity,
    gamepad_id: usize,
}

/// A function that is run on GamepadConnection.Connected GamepadConnectionEvents to
/// add the connected gamepad to the first player in the player_entities_without_controllers
/// query.
/// This function also works if the gamepad is connected to the machine before the
/// program starts because on program startup, the player character is added before
/// checking for connected gamepads (the check for connected gamepads when the program
/// starts issues a connection event for all gamepads connected with the machine before
/// the program started).
///
/// TODO: It probably makes sense to change the logic here. The current logic expects
/// a player to already be created, which is fine for this early development. But, in
/// the long run, it probably makes sense for a controller connection event to spawn a
/// player entity. That way a player can drop in rather than needing to pick the number
/// of players when launching the game.
/// A consequence of this would probably needing to spawn a generic player that loads
/// from saved data or created from new. Another consequence of this would be
/// potentially trying to connect to an existing player for an in-progress game (like
/// the gamepad dies and connecting a new one to keep playing).
fn connect_controller_to_player(
    commands: &mut Commands,
    connection_event: &GamepadConnectionEvent,
    gamepad_info: &GamepadInfo,
    player_entities_without_controllers: &Query<(Entity, &PlayerCharacter), Without<Controller>>,
) {
    if let Some((player_entity, player)) = player_entities_without_controllers.into_iter().next() {
        commands.entity(player_entity).insert(Controller {
            gamepad: connection_event.gamepad,
        });
        info!(
            "Gamepad {} of id {} assigned to player with id {}",
            gamepad_info.name, connection_event.gamepad.id, player.id
        );
    }
}

/// A function that is run on GamepadConnection.Disconnected GamepadConnectionEvents to
/// remove the disconnected gamepad from the first player in the
/// player_entities_with_controllers query that has a gamepad with an id that matches the id of
/// the gamepad that disconnected.
///
/// TODO: Make this an updated corollary to the connected function. This function would
/// despawn the player for the gamepad that was disconnected. There also might need to
/// be some logic to deal with gamepads that disconnect due to a lose of power.
fn disconnect_controller_from_player(
    commands: &mut Commands,
    connection_event: &GamepadConnectionEvent,
    player_entities_with_controllers: &Query<(Entity, &PlayerCharacter, &Controller)>,
) {
    for (player_entity, player, controller) in player_entities_with_controllers {
        if controller.gamepad.id == connection_event.gamepad.id {
            commands.entity(player_entity).remove::<Controller>();
            info!(
                "Controller with gamepad of id {} removed from player with id {}",
                connection_event.gamepad.id, player.id
            );
            break;
        }
    }
}

/// A system that listens to the gamepad connection events and then executes
/// the appropriate function based on the event type.
pub fn gamepad_connection_events(
    mut commands: Commands,
    mut connection_events: EventReader<GamepadConnectionEvent>,
    player_entities_with_controllers: Query<(Entity, &PlayerCharacter, &Controller)>,
    player_entities_without_controllers: Query<(Entity, &PlayerCharacter), Without<Controller>>,
) {
    for connection_event in connection_events.iter() {
        match &connection_event.connection {
            Connected(gamepad_info) => connect_controller_to_player(
                &mut commands,
                connection_event,
                gamepad_info,
                &player_entities_without_controllers,
            ),
            Disconnected => disconnect_controller_from_player(
                &mut commands,
                connection_event,
                &player_entities_with_controllers,
            ),
        }
    }
}

/// Return the entity and gamepad id for the requested player.
fn get_player_entity_and_gamepad_id(
    player_id: u8,
    players_with_controller: Query<(Entity, &PlayerCharacter, &Controller)>,
) -> Option<PlayerInfo> {
    let mut player_data = None;
    for (player_entity, player, controller) in players_with_controller.iter() {
        if player.id == player_id {
            player_data = Some(PlayerInfo {
                entity: player_entity,
                gamepad_id: controller.gamepad.id,
            });
            // Found the info for the player of interest so stop.
            break;
        }
    }
    return player_data;
}

/// Returns the requested gamepad.
fn get_gamepad(gamepad_id: usize, gamepads: Res<Gamepads>) -> Option<Gamepad> {
    let mut gamepad_returned = None;
    for gamepad in gamepads.iter() {
        if gamepad.id == gamepad_id {
            gamepad_returned = Some(gamepad);
            // Found the gamepad we want so stop.
            break;
        }
    }
    return gamepad_returned;
}

/// Gets the axis state of the gamepad and uses it to calculate a displacement
/// vector for the frame.
fn calculate_displacement_vector(
    gamepad: Gamepad,
    axes: Res<Axis<GamepadAxis>>,
    timer: Res<Time>,
) -> Option<Vec3> {
    let mut displacement_vector = None;
    let speed = 10.0;
    let x_axis = GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX);
    let y_axis = GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY);
    // Get the axis information from the gamepad and move the
    // player's entity based on the axis info.
    if let (Some(x_axis), Some(y_axis)) = (axes.get(x_axis), axes.get(y_axis)) {
        let x_displacement = speed * y_axis * timer.delta_seconds();
        let z_displacement = speed * x_axis * timer.delta_seconds();
        displacement_vector = Some(Vec3::new(x_displacement, 0.0, z_displacement));
    }
    return displacement_vector;
}

/// Generates a system to move a player entity's transform based on the input from the
/// player's gamepad's left stick.
pub fn generate_move_player_system(
    player_id: u8,
) -> impl Fn(
    Res<Gamepads>,
    Res<Axis<GamepadAxis>>,
    Query<(Entity, &PlayerCharacter, &Controller)>,
    Query<&mut Transform>,
    Res<Time>,
) {
    // This closure is the system that is generated.
    move |gamepads: Res<Gamepads>,
          axes: Res<Axis<GamepadAxis>>,
          players_with_controller: Query<(Entity, &PlayerCharacter, &Controller)>,
          mut transforms: Query<&mut Transform>,
          timer: Res<Time>| {
        if let Some(player_info) =
            get_player_entity_and_gamepad_id(player_id, players_with_controller)
        {
            if let Some(gamepad) = get_gamepad(player_info.gamepad_id, gamepads) {
                if let Some(displacement) = calculate_displacement_vector(gamepad, axes, timer) {
                    if let Ok(mut player_transform) = transforms.get_mut(player_info.entity) {
                        player_transform.translation += displacement;
                    }
                }
            }
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