use std::sync::Arc;

use bevy::{
    asset::Handle,
    ecs::{
        bundle::Bundle,
        component::Component,
        entity::Entity,
        lifecycle::HookContext,
        message::{MessageReader, MessageWriter},
        name::Name,
        query::With,
        reflect::ReflectComponent,
        system::{Commands, Query, Res, ResMut, Single},
        world::{DeferredWorld, Mut, Ref, World},
    },
    log,
    math::Vec2,
    prelude::{Deref, DerefMut},
    reflect::Reflect,
    sprite::Sprite,
    time::{Time, Virtual},
    transform::components::Transform,
};
use bevy_mod_scripting::{
    asset::ScriptAsset,
    bindings::{AppReflectAllocator, ReflectAllocator, ReflectReference},
    prelude::{ScriptCallbackEvent, ScriptValue},
};
use bevy_rapier2d::{
    parry::query::contact,
    prelude::{
        ActiveCollisionTypes, ActiveEvents, Collider, CollisionEvent, CollisionGroups,
        RapierColliderHandle, RapierContextSimulation, RigidBody, Sensor, Velocity,
    },
    rapier::prelude::{ColliderHandle, NarrowPhase},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    character::controllable_character::Character,
    mods::mod_descriptor_asset::ModPathBuf,
    physics::CollisionGroup,
    spells::{executor::AbilityExecutionId, spell_component_asset::SpellComponentAsset},
};

#[derive(Component)]
pub struct WithLifetime {
    /// The elapsed wrapping virtual time at which this lifetime was started
    pub start_at: f64,
    /// the time in seconds to keep this lifetime for
    pub lifetime_seconds: f64,
    /// if true is already expired, and will be despawned next frame
    pub expired: bool,
}

impl WithLifetime {
    pub fn new(time: &Time<Virtual>, lifetime_seconds: f64) -> Self {
        Self {
            start_at: time.elapsed_secs_wrapped_f64(),
            lifetime_seconds,
            expired: false,
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ExecutingSpellComponent {
    pub descriptor: Handle<SpellComponentAsset>,
    pub execution_id: AbilityExecutionId,
}

/// A spell component instantiated
#[derive(Bundle)]
pub struct LiveSpell {
    pub spell_component: ExecutingSpellComponent,
    pub lifetime: WithLifetime,
    pub colider: Collider,
    pub collision_flags: ActiveCollisionTypes,
    pub active_collision_events: ActiveEvents,
    pub collision_groups: CollisionGroups,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub sprite: Sprite,
    pub transform: Transform,
    /// mostly for the purposes of the inspector
    pub name: Name,
}

impl LiveSpell {
    /// Creates a new default set of components for a live spell.
    /// By default these are:
    /// - transform, collider and rigid body with a 0.1m diameter
    /// - a starting velocity
    /// - A sprite
    pub fn new(
        execution_id: AbilityExecutionId,
        position: Vec2,
        velocity: Vec2,
        time: &Time<Virtual>,
        handle: Handle<SpellComponentAsset>,
        descriptor: &SpellComponentAsset,
    ) -> Self {
        let diameter = 0.1;
        let (collision_groups, active_collision_events) = match descriptor
            .descriptor
            .disable_collisions
        {
            false => (
                CollisionGroups::new(
                    CollisionGroup::Projectile.into(),
                    [
                        CollisionGroup::ControlledEntity.into(),
                        CollisionGroup::Terrain.into(),
                    ]
                    .into_iter()
                    .collect(),
                ),
                ActiveEvents::COLLISION_EVENTS,
            ),
            true => (
                CollisionGroups::new(CollisionGroup::Projectile.into(), [].into_iter().collect()),
                ActiveEvents::empty(),
            ),
        };
        Self {
            name: Name::new(descriptor.descriptor.identifier.clone()),
            lifetime: WithLifetime {
                start_at: time.elapsed_secs_wrapped_f64(),
                expired: false,
                lifetime_seconds: descriptor.descriptor.lifetime_milliseconds as f64 / 1000.,
            },
            spell_component: ExecutingSpellComponent {
                descriptor: handle,
                execution_id,
            },
            colider: Collider::ball(diameter / 2.),
            rigid_body: RigidBody::Dynamic,
            sprite: Sprite {
                custom_size: Some(Vec2 {
                    x: diameter,
                    y: diameter,
                }),
                ..Default::default()
            },
            velocity: Velocity::linear(velocity),
            transform: Transform::from_xyz(position.x, position.y, 0.),
            collision_flags: ActiveCollisionTypes::all(),
            collision_groups,
            active_collision_events,
        }
    }
}
