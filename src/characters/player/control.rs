use bevy::prelude::{Component, Entity, Gamepad, Gamepads, Query, Res};

use super::camera::components::PlayerCamera;
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

/// Returns the entity for the camera associated with the provide player id.
pub fn get_player_camera_entity(
    player_id: u8,
    player_cameras: Query<(Entity, &PlayerCamera)>,
) -> Option<Entity> {
    let mut entity_returned = None;
    for (camera_entity, player_camera) in player_cameras.iter() {
        if player_camera.player_id == player_id {
            entity_returned = Some(camera_entity);
            // Found the camera associated with the player we want so stop.
            break;
        }
    }
    return entity_returned;
}
