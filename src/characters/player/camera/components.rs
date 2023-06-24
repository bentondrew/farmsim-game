use bevy::prelude::Component;

/// A component to put on a camera entity to associate it with a player.
#[derive(Component)]
pub struct PlayerCamera {
    pub player_id: u8,
}
