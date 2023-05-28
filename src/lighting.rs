use bevy::prelude::{default, Commands, PointLight, PointLightBundle, Transform};

pub fn add_light(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 600000.,
            range: 100.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(50., 50., 50.),
        ..default()
    });
}
