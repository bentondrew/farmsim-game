use bevy::prelude::Component;

/// Components that are common to different character types.

/// A component to give an entity a name.
#[derive(Component)]
pub struct Name(pub String);

/// A component to indicate if an entity is not a player  character.
#[derive(Component)]
pub struct NonPlayerCharacter;
