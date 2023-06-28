use std::f32::consts::PI;

use bevy::prelude::{
    Axis, Entity, Gamepad, GamepadAxis, GamepadAxisType, Gamepads, Mat3, Quat, Query, Res, Time,
    Transform, Vec3,
};

use super::super::control::{get_gamepad, get_player_entity_and_gamepad_id, Controller};
use super::super::entity::components::PlayerCharacter;
use super::components::PlayerCamera;

/// Returns the entity for the camera associated with the provide player id..
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

fn calculate_rotation(
    gamepad: Gamepad,
    axes: Res<Axis<GamepadAxis>>,
    timer: Res<Time>,
) -> Option<Quat> {
    let mut rotation: Option<Quat> = None;
    let speed = 1.0;
    let x_axis = GamepadAxis::new(gamepad, GamepadAxisType::RightStickX);
    let y_axis = GamepadAxis::new(gamepad, GamepadAxisType::RightStickY);
    if let (Some(x_axis), Some(y_axis)) = (axes.get(x_axis), axes.get(y_axis)) {
        let percent_y_rotation = speed * x_axis * timer.delta_seconds();
        let y_rotation = Quat::from_rotation_y(percent_y_rotation * 2.0 * PI);
        let percent_x_rotation = speed * y_axis * timer.delta_seconds();
        let x_rotation = Quat::from_rotation_x(percent_x_rotation * PI);
        rotation = Some(y_rotation * x_rotation);
    }
    return rotation;
}

pub fn generate_move_player_camera_system(
    player_id: u8,
) -> impl Fn(
    Res<Gamepads>,
    Res<Axis<GamepadAxis>>,
    Query<(Entity, &PlayerCharacter, &Controller)>,
    Query<(Entity, &PlayerCamera)>,
    Query<&mut Transform>,
    Res<Time>,
) {
    move |gamepads: Res<Gamepads>,
          axes: Res<Axis<GamepadAxis>>,
          players_with_controller: Query<(Entity, &PlayerCharacter, &Controller)>,
          player_cameras: Query<(Entity, &PlayerCamera)>,
          mut transforms: Query<&mut Transform>,
          timer: Res<Time>| {
        if let Some(player_info) =
            get_player_entity_and_gamepad_id(player_id, players_with_controller)
        {
            if let Some(gamepad) = get_gamepad(player_info.gamepad_id, gamepads) {
                if let Some(camera_entity) = get_player_camera_entity(player_id, player_cameras) {
                    if let Some(rotation) = calculate_rotation(gamepad, axes, timer) {
                        if let Ok(mut camera_transform) = transforms.get_mut(camera_entity) {
                            // Apply the rotation to the vector
                            let rotation_matrix = Mat3::from_quat(rotation);
                            camera_transform.translation =
                                rotation_matrix.mul_vec3(camera_transform.translation);
                            // Now look at center
                            let refocused_transform =
                                camera_transform.looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y);
                            camera_transform.translation = refocused_transform.translation;
                            camera_transform.rotation = refocused_transform.rotation;
                            camera_transform.scale = refocused_transform.scale;
                        }
                    }
                }
            }
        }
    }
}
