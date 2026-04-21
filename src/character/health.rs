use bevy::{
    ecs::{component::Component, reflect::ReflectComponent},
    reflect::Reflect,
};

#[derive(Reflect, Component)]
#[reflect(Component)]
pub struct Health {
    /// The full amount of health this entity can have
    pub maximum: f32,
    /// The current amount of health this entity has
    pub current: f32,
}
