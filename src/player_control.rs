use bevy::{
    input::gamepad::{
        GamepadAxisChangedEvent,
        GamepadConnection::{Connected, Disconnected},
        GamepadConnectionEvent, GamepadInfo,
    },
    prelude::{info, Commands, Component, Entity, EventReader, Gamepad, Query, With, Without},
};

use crate::characters::common::{Name, PlayerCharacter};

#[derive(Component)]
pub struct Controller {
    gamepad: Gamepad,
}

fn connect_controller_to_player(
    commands: &mut Commands,
    connection_event: &GamepadConnectionEvent,
    gamepad_info: &GamepadInfo,
    players_without_controllers: &Query<
        (Entity, &Name),
        (With<PlayerCharacter>, Without<Controller>),
    >,
) {
    // For the first player entity in the query, add the connected controller.
    // This also works on startup as the gamepad resources are added after
    // creating a player and the gamepad connection events are caught for gamepads that
    // are already on when the game is started.
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

fn disconnect_controller_from_player(
    commands: &mut Commands,
    connection_event: &GamepadConnectionEvent,
    players_with_controllers: &Query<
        (Entity, &Name, &Controller),
        (With<PlayerCharacter>, With<Controller>),
    >,
) {
    // Find the first entity with a controlled with a gamepad id
    // matching the gamepad id from the event, then remove the
    // controller from that entity.
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

pub fn gamepad_axis_changed_events(mut axis_events: EventReader<GamepadAxisChangedEvent>) {
    for axis_event in axis_events.iter() {
        info!(
            "{:?} of {:?} is changed to {}",
            axis_event.axis_type, axis_event.gamepad, axis_event.value,
        );
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
