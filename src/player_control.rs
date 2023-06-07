use bevy::{
    input::gamepad::{
        GamepadAxisChangedEvent,
        GamepadConnection::{Connected, Disconnected},
        GamepadConnectionEvent, GamepadInfo,
    },
    prelude::{info, Commands, Component, Entity, EventReader, Gamepad, Query, Without},
};

use crate::characters::players::PlayerCharacter;

/// A Bevy Engine component that is attached to an entity that represents the resource
/// that controls that entity. Expected to be attached to entities that also have the
/// Player component. Currently only supports Gamepad controls.
#[derive(Component)]
pub struct Controller {
    gamepad: Gamepad,
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

/// Generates a system for the provided player id that listens to the
/// GamepadAxisChangedEvent events for the gamepad associated with that player.
///
/// TODO: The intent of this function is to filter the events based on the player
/// id to get the events associated with that player's gamepad. This function will
/// initially be used to get the left stick events to move the player. Additional
/// systems will need to be created to capture the right stick events to move the
/// camera and capture the button presses.
pub fn generate_players_gamepad_left_stick_events_system(
    player_id: u8,
) -> impl Fn(EventReader<GamepadAxisChangedEvent>, Query<(&PlayerCharacter, &Controller)>) {
    // TODO: Once the events are filtered for the player, filter to left stick events.

    // TODO: Once events are down filtered for the player's gamepad's left stick events,
    // apply the events to the player's entity transformations.
    move |mut axis_events: EventReader<GamepadAxisChangedEvent>,
          players_with_controllers: Query<(&PlayerCharacter, &Controller)>| {
        let mut player_gamepad: Option<Gamepad> = None;
        for (player, controller) in players_with_controllers.iter() {
            if player.id == player_id {
                player_gamepad = Some(controller.gamepad);
            }
        }
        for axis_event in axis_events.iter() {
            info!("Handling events for player {}", player_id);
            let _result = match player_gamepad {
                Some(gamepad) => {
                    info!("Player's gamepad has id {}", gamepad.id);
                    if gamepad.id == axis_event.gamepad.id {
                        info!(
                            "{:?} of {:?} is changed to {}",
                            axis_event.axis_type, axis_event.gamepad, axis_event.value,
                        );
                    };
                }
                None => (),
            };
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
