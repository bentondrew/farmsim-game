use std::f32::consts::PI;

use bevy::prelude::{
    Axis, Entity, Gamepad, GamepadAxis, GamepadAxisType, Gamepads, Mat3, Quat, Query, Res, Time,
    Transform, Vec3,
};

use super::super::control::{player_gamepad_movement_wrapper, Controller, PlayerInfo};
use super::super::entity::components::PlayerCharacter;
use super::components::PlayerCamera;

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

fn move_camera(
    axes: Res<Axis<GamepadAxis>>,
    mut transforms: Query<&mut Transform>,
    timer: Res<Time>,
    _player_info: PlayerInfo,
    gamepad: Gamepad,
    camera_entity: Entity,
) {
    if let Some(rotation) = calculate_rotation(gamepad, axes, timer) {
        if let Ok(mut camera_transform) = transforms.get_mut(camera_entity) {
            // Apply the rotation to the vector
            let rotation_matrix = Mat3::from_quat(rotation);
            camera_transform.translation = rotation_matrix.mul_vec3(camera_transform.translation);
            // Now look at center
            let refocused_transform =
                camera_transform.looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y);
            camera_transform.translation = refocused_transform.translation;
            camera_transform.rotation = refocused_transform.rotation;
            camera_transform.scale = refocused_transform.scale;
        }
    }
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
    return player_gamepad_movement_wrapper(player_id, move_camera);
}
