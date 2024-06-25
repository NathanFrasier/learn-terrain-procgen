use bevy::prelude::*;

#[derive(Component)]
pub struct GameCamera;

pub fn camera_orbit(mut query: Query<&mut Transform, With<GameCamera>>, time: Res<Time>) {
    if let Ok(mut transform) = query.get_single_mut() {
        transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(time.delta_seconds() / 2.));
    }
}
