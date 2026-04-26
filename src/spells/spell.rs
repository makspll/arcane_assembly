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
    character::controllable_character::Character, mods::mod_descriptor_asset::ModPathBuf,
    physics::CollisionGroup, spells::executor::AbilityExecutionId,
};

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, Default, Reflect)]
pub enum SlotCount {
    #[default]
    None,
    Unlimited,
    FixedAmount(usize),
}

/// A reference to a spell component descriptor, used to avoid cloning excessively, and reduce stack size of structs
#[derive(Reflect, Clone, Deref, DerefMut, Debug)]
pub struct SpellComponentDescriptorHandle(pub Arc<SpellComponentDescriptor>);

impl From<SpellComponentDescriptor> for SpellComponentDescriptorHandle {
    fn from(value: SpellComponentDescriptor) -> Self {
        Self(Arc::new(value))
    }
}

/// A descriptor for a
#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, Reflect)]
pub struct SpellComponentDescriptor {
    /// The name to show in the UI
    pub friendly_name: String,
    /// The postfix to give to every handler for this spell. For example if this name is 'my_spell' then the handler for casting the spell
    /// would have to be called "on_cast_my_spell", and similar for all the other spell callbacks.
    pub handler_label: String,
    /// the path to an icon within the mod that can be shown in the UI, if not provided a placeholder will be used
    pub icon_sprite_path: Option<ModPathBuf>,
    /// The controller script for this component.
    /// All callbacks and triggers get sent to it while the component is alive
    pub script_controller_path: ModPathBuf,
    #[serde(skip_deserializing, skip_serializing, default)]
    pub script_controller_handle: Option<Handle<ScriptAsset>>,
    /// The amount of mana drained every time this component is triggered.
    /// If mana is not enough for the next component, firing is blocked.
    pub mana_drain_per_shot: f32,
    /// The delay after which the next component can be triggered following this one.
    /// If less than the time between frames, will trigger a
    /// component every frame, but no more frequently than that.
    pub delay_milliseconds: f32,
    /// The time after which this component is killed, and its death effect triggered
    pub lifetime_milliseconds: f32,
    /// Components can be slotted with children components, for example
    /// a grouping component might trigger effects within its children components every frame
    #[serde(default)]
    pub children_slots: SlotCount,
    /// If a component has an area of effect, the engine will provide nearest entities/projectiles to its callbacks
    pub area_of_effect_meters: f32,
}

impl std::fmt::Debug for SpellComponentDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpellComponentDescriptor")
            .field("friendly_name", &self.friendly_name)
            .field("handler_label", &self.handler_label)
            .field("script_controller_path", &self.script_controller_path)
            .finish()
    }
}

#[derive(Component)]
pub struct WithLifetime {
    /// The elapsed wrapping virtual time at which this lifetime was started
    pub start_at: f64,
    /// the time in seconds to keep this lifetime for
    pub lifetime_seconds: f64,
    /// if true is already expired
    pub expired: bool,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ExecutingSpellComponent {
    pub descriptor: SpellComponentDescriptorHandle,
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
        descriptor: SpellComponentDescriptorHandle,
    ) -> Self {
        let diameter = 0.1;
        Self {
            name: Name::new(descriptor.friendly_name.clone()),
            lifetime: WithLifetime {
                start_at: time.elapsed_secs_wrapped_f64(),
                expired: false,
                lifetime_seconds: descriptor.lifetime_milliseconds as f64 / 1000.,
            },
            spell_component: ExecutingSpellComponent {
                descriptor,
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
            collision_groups: CollisionGroups::new(
                CollisionGroup::Projectile.into(),
                [
                    CollisionGroup::ControlledEntity.into(),
                    CollisionGroup::Terrain.into(),
                ]
                .into_iter()
                .collect(),
            ),
            active_collision_events: ActiveEvents::COLLISION_EVENTS,
        }
    }
}
