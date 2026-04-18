use bevy::{app::Plugin, math::Vec2};
use bevy_rapier2d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};

pub const UP: Vec2 = Vec2 { x: 0.0, y: 1.0 };
pub const DOWN: Vec2 = Vec2 { x: 0.0, y: -1.0 };
pub const LEFT: Vec2 = Vec2 { x: -1.0, y: 0.0 };
pub const RIGHT: Vec2 = Vec2 { x: 1.0, y: 0.0 };

pub const PIXELS_PER_METER: f32 = 32.0;
pub const METERS_PER_PIXEL: f32 = 1. / PIXELS_PER_METER;
pub const GRAVITY_ACCELERATION_IN_METERS: f32 = -9.8;
pub const GRAVITY_ACCELERATION_IN_PIXELS: f32 = -9.8 * PIXELS_PER_METER;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            PIXELS_PER_METER,
        ))
        .add_plugins(RapierDebugRenderPlugin::default());
    }
}
