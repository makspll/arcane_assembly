use bevy::{app::Plugin, ecs::bundle::Bundle, math::Vec2};
use bevy_rapier2d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    prelude::{Collider, CollisionGroups, Group},
    rapier::prelude::RigidBody,
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

/// Available collision groups in the game
pub enum CollisionGroup {
    Projectile,
    ControlledEntity,
    Terrain,
}

impl Into<Group> for CollisionGroup {
    fn into(self) -> Group {
        match self {
            CollisionGroup::Projectile => Group::GROUP_1,
            CollisionGroup::ControlledEntity => Group::GROUP_2,
            CollisionGroup::Terrain => Group::GROUP_3,
        }
    }
}

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            PIXELS_PER_METER,
        ));

        #[cfg(feature = "dev_tools")]
        {
            use bevy_rapier2d::render::RapierDebugRenderPlugin;

            app.add_plugins(RapierDebugRenderPlugin::default());
        }
    }
}
