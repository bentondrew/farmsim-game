use bevy::prelude::{Component, Entity, Gamepad, Gamepads, Query, Res};

use super::entity::components::PlayerCharacter;

/// A Bevy Engine component that is attached to an entity that represents the resource
/// that controls that entity. Expected to be attached to entities that also have the
/// Player component. Currently only supports Gamepad controls.
#[derive(Component)]
pub struct Controller {
    pub gamepad: Gamepad,
}

/// Contains player information for determining player movement.
pub struct PlayerInfo {
    pub entity: Entity,
    pub gamepad_id: usize,
}

/// Return the entity and gamepad id for the requested player.
pub fn get_player_entity_and_gamepad_id(
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
pub fn get_gamepad(gamepad_id: usize, gamepads: Res<Gamepads>) -> Option<Gamepad> {
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
