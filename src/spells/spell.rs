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
        system::{Commands, Query, Res, ResMut, Single},
        world::{DeferredWorld, Mut, Ref, World},
    },
    log,
    math::Vec2,
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
    physics::CollisionGroup,
    scripts::script_descriptor::ModPathBuf,
    spells::callbacks::{OnSpellCast, OnSpellExpired, OnSpellHitCharacter, OnSpellHitTerrain},
};

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, Default)]
pub enum SlotCount {
    #[default]
    None,
    Unlimited,
    FixedAmount(usize),
}

/// A descriptor for a
#[derive(Component, Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[component(on_add = Self::on_component_removed)]
pub struct SpellComponentDescriptor {
    /// The name to show in the UI
    pub friendly_name: String,
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

impl SpellComponentDescriptor {
    pub fn on_component_removed(world: DeferredWorld, context: HookContext) {
        Self::on_cast(world, context.entity)
    }

    pub fn on_cast(mut world: DeferredWorld, entity: Entity) {
        if let Ok(entity_ref) = world.get_entity(entity)
            && let Some(descriptor) = entity_ref.get::<Self>()
            && let Some(controller) = &descriptor.script_controller_handle
        {
            let allocator_guard = world.resource::<AppReflectAllocator>();
            let mut allocator = allocator_guard.write();
            let entity = ReflectReference::new_allocated(entity_ref.id(), &mut allocator);
            drop(allocator);
            world.write_message(ScriptCallbackEvent::new_for_static_script(
                OnSpellCast,
                vec![ScriptValue::Reference(entity)],
                controller.clone(),
            ));
        } else {
            log::error!(
                "spell inserted but could not identify controller script. will not trigger insert callback.",
            );
        }
    }

    pub fn on_expire(&self, commands: &mut Commands, entity: Entity) {
        if let Some(controller) = self.script_controller_handle.clone() {
            commands.queue(move |world: &mut World| {
                let allocator_guard = world.resource::<AppReflectAllocator>();
                let mut allocator = allocator_guard.write();
                let entity = ReflectReference::new_allocated(entity, &mut allocator);
                drop(allocator);
                world.write_message(ScriptCallbackEvent::new_for_static_script(
                    OnSpellExpired,
                    vec![ScriptValue::Reference(entity)],
                    controller.clone(),
                ));
            });
        } else {
            log::error!(
                "spell expired but could not identify controller script. Will not trigger expire callback.",
            );
        }
    }

    pub fn on_collision(&self, commands: &mut Commands, entity: Entity, other_entity: Entity) {
        if let Some(controller) = self.script_controller_handle.clone() {
            commands.queue(move |world: &mut World| {
                if let Ok(other_entity_ref) = world.get_entity(other_entity) {
                    let allocator_guard = world.resource::<AppReflectAllocator>();
                    let mut allocator = allocator_guard.write();
                    let entity = ReflectReference::new_allocated(entity, &mut allocator);
                    let other_entity =
                        ReflectReference::new_allocated(other_entity, &mut allocator);
                    drop(allocator);
                    let payload = vec![
                        ScriptValue::Reference(entity),
                        ScriptValue::Reference(other_entity),
                    ];
                    if other_entity_ref.contains::<Character>() {
                        world.write_message(ScriptCallbackEvent::new_for_static_script(
                            OnSpellHitCharacter,
                            payload,
                            controller,
                        ));
                    } else {
                        world.write_message(ScriptCallbackEvent::new_for_static_script(
                            OnSpellHitTerrain,
                            payload,
                            controller,
                        ));
                    }
                }
            });
        } else {
            log::error!(
                "spell hit triggered but could not identify controller script. Will not trigger hit callback.",
            );
        }
    }
}

#[derive(Component)]
pub struct WithLifetime {
    /// The elapsed wrapping virtual time at which this lifetime was started
    start_at: f64,
    /// the time in seconds to keep this lifetime for
    lifetime_seconds: f64,
    /// if true is already expired
    expired: bool,
}

/// A spell component instantiated
#[derive(Bundle)]
pub struct LiveSpell {
    pub descriptor: SpellComponentDescriptor,
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
        position: Vec2,
        velocity: Vec2,
        time: &Time<Virtual>,
        descriptor: SpellComponentDescriptor,
    ) -> Self {
        let diameter = 0.1;
        Self {
            name: Name::new(descriptor.friendly_name.clone()),
            lifetime: WithLifetime {
                start_at: time.elapsed_secs_wrapped_f64(),
                expired: false,
                lifetime_seconds: descriptor.lifetime_milliseconds as f64 / 1000.,
            },
            descriptor,

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

pub fn trigger_spell_expirations(
    mut commands: Commands,
    time: Res<Time<Virtual>>,
    mut spells: Query<(Entity, Ref<SpellComponentDescriptor>, Mut<WithLifetime>)>,
) {
    let current_time = time.elapsed_secs_wrapped_f64();

    for (entity, spell, mut lifetime) in spells.iter_mut() {
        if lifetime.expired {
            continue;
        }

        // abs due to possible wrapping
        let diff = (lifetime.start_at - current_time).abs();
        if diff <= lifetime.lifetime_seconds {
            continue;
        }

        lifetime.expired = true;

        spell.on_expire(&mut commands, entity);
    }
}

// rapier also does Contact events, but those require keeping track of entities we care about in map or something.
// here because we know these are short lived, it might be better to go from the projectiles themselvevs, and check.
// the collision graph is gonna be hella optiimzed for this.
pub fn trigger_spell_hits(
    mut commands: Commands,
    collidable_spells: Query<
        (Entity, Ref<SpellComponentDescriptor>),
        With<SpellComponentDescriptor>,
    >,
    mut collisions: MessageReader<CollisionEvent>,
    // mut physics_context: Single<Ref<RapierContextSimulation>>,
) {
    // there is also CollidingEntities as a component, which can be inserted and rapier will do this for us
    // but we'd need to manually workout which collisions are new each frame
    for collision in collisions.read() {
        // do something on stopped too ?
        if let CollisionEvent::Started(e1, e2, _) = collision {
            let (main_entity, other_entity, spell) =
                if let Ok((entity, spell)) = collidable_spells.get(*e1) {
                    (entity, e2, spell)
                } else if let Ok((entity, spell)) = collidable_spells.get(*e2) {
                    (entity, e1, spell)
                } else {
                    continue;
                };
            spell.on_collision(&mut commands, main_entity, *other_entity);
        }
    }
}
