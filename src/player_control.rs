use bevy::{
    input::gamepad::{
        GamepadAxisChangedEvent,
        GamepadConnection::{Connected, Disconnected},
        GamepadConnectionEvent, GamepadInfo,
    },
    prelude::{info, Commands, Component, Entity, EventReader, Gamepad, Query, With, Without},
};

use crate::characters::{common::Name, players::PlayerCharacter};

/// A Bevy Engine component that is attached to an entity that represents the resource
/// that controls that entity. Expected to be attached to entities that also have the
/// Player component. Currently only supports Gamepad controls.
#[derive(Component)]
pub struct Controller {
    gamepad: Gamepad,
}

/// A function that is run on GamepadConnection.Connected GamepadConnectionEvents to
/// add the connected gamepad to the first player in the players_without_controllers
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
    players_without_controllers: &Query<
        (Entity, &Name),
        (With<PlayerCharacter>, Without<Controller>),
    >,
) {
    if let Some((player_entity, player_name)) = players_without_controllers.into_iter().next() {
        commands.entity(player_entity).insert(Controller {
            gamepad: connection_event.gamepad,
        });
        info!(
            "Gamepad {} of id {} assigned to player {}",
            gamepad_info.name, connection_event.gamepad.id, player_name.0
        );
    }
}

/// A function that is run on GamepadConnection.Disconnected GamepadConnectionEvents to
/// remove the disconnected gamepad from the first player in the
/// players_with_controllers query that has a gamepad with an id that matches the id of
/// the gamepad that disconnected.
///
/// TODO: Make this an updated corollary to the connected function. This function would
/// despawn the player for the gamepad that was disconnected. There also might need to
/// be some logic to deal with gamepads that disconnect due to a lose of power.
fn disconnect_controller_from_player(
    commands: &mut Commands,
    connection_event: &GamepadConnectionEvent,
    players_with_controllers: &Query<
        (Entity, &Name, &Controller),
        (With<PlayerCharacter>, With<Controller>),
    >,
) {
    for (player_entity, player_name, controller) in players_with_controllers {
        if controller.gamepad.id == connection_event.gamepad.id {
            commands.entity(player_entity).remove::<Controller>();
            info!(
                "Controller with gamepad of id {} removed from player {}",
                connection_event.gamepad.id, player_name.0
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
    players_with_controllers: Query<
        (Entity, &Name, &Controller),
        (With<PlayerCharacter>, With<Controller>),
    >,
    players_without_controllers: Query<
        (Entity, &Name),
        (With<PlayerCharacter>, Without<Controller>),
    >,
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
                &players_with_controllers,
            ),
        }
    }
}

/// Generates a system for the provided player id that listens to the
/// GamepadAxisChangedEvent events.
///
/// TODO: The intent of this function is to filter the events based on the player
/// id to get the events associated with that player's gamepad. This function will
/// initially be used to get the left stick events to move the player. Additional
/// systems will need to be created to capture the right stick events to move the
/// camera and capture the button presses.
pub fn players_gamepad_axis_changed_events(
    player_id: u8,
) -> impl Fn(EventReader<GamepadAxisChangedEvent>) {
    move |mut axis_events: EventReader<GamepadAxisChangedEvent>| {
        for axis_event in axis_events.iter() {
            info!("Handling events for player {}", player_id);
            info!(
                "{:?} of {:?} is changed to {}",
                axis_event.axis_type, axis_event.gamepad, axis_event.value,
            );
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
