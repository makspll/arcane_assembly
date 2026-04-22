use crate::{character::controllable_character::Player, physics::METERS_PER_PIXEL};
use bevy::{
    app::{Plugin, Startup, Update},
    camera::{Camera, Camera2d, OrthographicProjection, Projection},
    ecs::{
        component::Component,
        query::{With, Without},
        reflect::ReflectComponent,
        system::{Commands, Res, Single},
    },
    math::{StableInterpolate, Vec3},
    reflect::Reflect,
    time::Time,
    transform::components::Transform,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, update_camera)
            .register_type::<MainCamera>()
            .register_type::<UiCamera>();
    }
}

/// How quickly should the camera snap to the desired location.
const CAMERA_DECAY_RATE: f32 = 10.;

/// Update the camera position by tracking the player.
fn update_camera(
    mut camera: Single<&mut Transform, (With<Camera2d>, With<MainCamera>, Without<Player>)>,
    player: Single<&Transform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
) {
    let Vec3 { x, y, .. } = player.translation;
    let direction = Vec3::new(x, y, camera.translation.z);

    camera
        .translation
        .smooth_nudge(&direction, CAMERA_DECAY_RATE, time.delta_secs());
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct MainCamera;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct UiCamera;

pub const MAIN_CAMERA_ORDER: usize = 0;
pub const UI_CAMERA_ORDER: usize = 10;
fn setup_camera(mut commands: Commands) {
    commands.spawn((
        MainCamera,
        Camera2d,
        Camera {
            order: MAIN_CAMERA_ORDER as isize,
            ..Default::default()
        },
        Transform::from_xyz(0., 0., 1.),
        Projection::from(OrthographicProjection {
            scale: METERS_PER_PIXEL * 0.1,
            ..OrthographicProjection::default_2d()
        }),
    ));
    commands.spawn((
        UiCamera,
        Camera2d,
        Camera {
            order: UI_CAMERA_ORDER as isize, // higher priority
            ..Default::default()
        },
    ));
}
