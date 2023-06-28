use bevy::{
    input::gamepad::{
        GamepadConnection::{Connected, Disconnected},
        GamepadConnectionEvent, GamepadInfo,
    },
    prelude::{
        info, Axis, Commands, Entity, EventReader, Gamepad, GamepadAxis, GamepadAxisType, Gamepads,
        Query, Res, Time, Transform, Vec3, Without,
    },
};

use super::super::camera::components::PlayerCamera;
use super::super::control::{player_gamepad_movement_wrapper, Controller, PlayerInfo};
use super::components::PlayerCharacter;

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

/// Gets the axis state of the left stick of the gamepad and uses it to calculate a
/// displacement vector for the frame. This displacement vector is relative to the
/// camera forward vector.
fn calculate_displacement_vector(
    gamepad: Gamepad,
    axes: Res<Axis<GamepadAxis>>,
    camera_transform: &Transform,
    player_transform: &Transform,
    timer: Res<Time>,
) -> Option<Vec3> {
    let mut displacement_vector = None;
    let speed = 10.0;
    let horizontal_axis = GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX);
    let forward_axis = GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY);
    if let (Some(horizontal_input), Some(forward_input)) =
        (axes.get(horizontal_axis), axes.get(forward_axis))
    {
        let right = camera_transform.right();
        // The camera forward cannot be used here as that means the player character
        // will go forward in the direction the camera is pointing. This means that the
        // player character is not constrained to the "ground". To constrain the
        // player character to the same plane it was on before by using using its up
        // and aligning the camera right with the player right.
        // This generates a z axis where the positive direction is toward the camera.
        let forward = right.cross(player_transform.up());
        let right_displacement = right * horizontal_input;
        // The desired behavior is that up on the left stick moves the player character
        // entity away from the camera so we need to flip the direction by using the
        // negative of the left stick Y as axis.
        let forward_displacement = forward * forward_input;
        let mut combined_displacement = right_displacement + forward_displacement * -1.0;
        combined_displacement *= speed;
        combined_displacement.clamp_length_max(speed);
        combined_displacement *= timer.delta_seconds();
        displacement_vector = Some(combined_displacement);
    }
    return displacement_vector;
}

/// Calculate a displacement based on the controller axes and apply that to the player
/// character entity.
fn move_entity(
    axes: Res<Axis<GamepadAxis>>,
    mut transforms: Query<&mut Transform>,
    timer: Res<Time>,
    player_info: PlayerInfo,
    gamepad: Gamepad,
    camera_entity: Entity,
) {
    let mut displacement = None;
    if let Ok(camera_transform) = transforms.get(camera_entity) {
        if let Ok(player_transform) = transforms.get(player_info.entity) {
            displacement = calculate_displacement_vector(
                gamepad,
                axes,
                &camera_transform,
                &player_transform,
                timer,
            )
        }
    }
    if let Ok(mut player_transform) = transforms.get_mut(player_info.entity) {
        match displacement {
            Some(vec) => player_transform.translation += vec,
            None => (),
        }
    }
}

/// Generates a system to move a player entity's transform based on the input from the
/// player's gamepad's left stick.
pub fn generate_move_player_system(
    player_id: u8,
) -> impl Fn(
    Res<Gamepads>,
    Res<Axis<GamepadAxis>>,
    Query<(Entity, &PlayerCharacter, &Controller)>,
    Query<(Entity, &PlayerCamera)>,
    Query<&mut Transform>,
    Res<Time>,
) {
    return player_gamepad_movement_wrapper(player_id, move_entity);
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
