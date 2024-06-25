//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::prelude::*;

mod camera;
mod map;
use camera::*;
use map::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Startup, generate_map)
        .add_systems(Update, camera_orbit)
        .run();
}

/// set up a simple 3D scene
fn setup(mut commands: Commands) {
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(5.0, 10.0, 5.0),
        ..default()
    });
    // camera
    commands.spawn((
        GameCamera,
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
    ));
}
