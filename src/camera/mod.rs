use bevy::{
    app::{Plugin, Startup, Update},
    camera::{Camera2d, OrthographicProjection, Projection, ScalingMode},
    ecs::{
        query::{With, Without},
        system::{Commands, Res, Single},
    },
    math::{StableInterpolate, Vec3},
    time::Time,
    transform::components::Transform,
};

use crate::character::controllable_character::Player;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, update_camera);
    }
}

/// How quickly should the camera snap to the desired location.
const CAMERA_DECAY_RATE: f32 = 10.;

/// Update the camera position by tracking the player.
fn update_camera(
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Single<&Transform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
) {
    let Vec3 { x, y, .. } = player.translation;
    let direction = Vec3::new(x, y, camera.translation.z);

    camera
        .translation
        .smooth_nudge(&direction, CAMERA_DECAY_RATE, time.delta_secs());
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Transform::from_xyz(0., 0., 1.),
        Projection::from(OrthographicProjection {
            scale: 0.1,
            ..OrthographicProjection::default_2d()
        }),
    ));
}
