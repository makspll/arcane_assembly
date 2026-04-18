use crate::physics::DOWN;
use bevy::{
    app::{Plugin, Startup},
    ecs::system::Commands,
    transform::components::Transform,
};
use bevy_rapier2d::prelude::Collider;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, init_flat_ground);
    }
}

fn init_flat_ground(mut commands: Commands) {
    commands.spawn((
        // 1 meter below default spawn
        Transform::from_translation((DOWN * 1.0).extend(0.0)),
        // 10 x 1 meters
        Collider::cuboid(5.0, 0.5),
    ));
}
// 19 seconds
