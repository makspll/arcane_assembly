use bevy::{
    ecs::{component::Component, reflect::ReflectComponent, system::EntityCommand},
    reflect::Reflect,
};

#[derive(Reflect, Component)]
#[reflect(Component)]
pub struct Mana {
    /// The full amount of mana this entity can have
    pub maximum: f32,
    /// The current amount of mana this entity has
    pub current: f32,
}
