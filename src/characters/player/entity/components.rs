use bevy::prelude::Component;

/// A component to indicate if an entity is a player character.
#[derive(Component)]
pub struct PlayerCharacter {
    pub id: u8,
    pub player_height_mid_point: f32,
}
