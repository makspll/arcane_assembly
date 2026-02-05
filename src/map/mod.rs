use bevy::{
    app::{Plugin, Startup},
    ecs::system::Commands,
    math::Vec3,
    transform::components::Transform,
};
use bevy_rapier2d::prelude::Collider;

use crate::physics::DOWN;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, init_flat_ground);
    }
}

fn init_flat_ground(mut commands: Commands) {
    commands.spawn((
        Transform::from_translation((DOWN * 50.0).extend(0.0)),
        Collider::cuboid(100., 10.),
    ));
}
