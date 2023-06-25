use std::f32::consts::PI;

use bevy::prelude::{
    info, Axis, Entity, Gamepad, GamepadAxis, GamepadAxisType, Gamepads, Quat, Query, Res, Time,
    Transform,
};

use super::super::control::{get_gamepad, get_player_entity_and_gamepad_id, Controller};
use super::super::entity::components::PlayerCharacter;

fn calculate_rotation(
    gamepad: Gamepad,
    axes: Res<Axis<GamepadAxis>>,
    timer: Res<Time>,
) -> Option<Quat> {
    let mut rotation: Option<Quat> = None;
    let speed = 10.0;
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
    Query<&mut Transform>,
    Res<Time>,
) {
    move |gamepads: Res<Gamepads>,
          axes: Res<Axis<GamepadAxis>>,
          players_with_controller: Query<(Entity, &PlayerCharacter, &Controller)>,
          mut transforms: Query<&mut Transform>,
          timer: Res<Time>| {
        if let Some(player_info) =
            get_player_entity_and_gamepad_id(player_id, players_with_controller)
        {
            if let Some(gamepad) = get_gamepad(player_info.gamepad_id, gamepads) {
                if let Some(rotation) = calculate_rotation(gamepad, axes, timer) {
                    info!("Calculated camera rotation: {}", rotation);
                }
            }
        }
    }
}
