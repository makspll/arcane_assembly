use bevy::{
    asset::Handle,
    ecs::{
        bundle::Bundle,
        component::Component,
        entity::Entity,
        message::MessageWriter,
        name::Name,
        system::{Commands, Query, Res, ResMut},
        world::Ref,
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
use bevy_rapier2d::prelude::{Collider, RigidBody, Velocity};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{scripts::script_descriptor::ModPathBuf, spells::callbacks::OnSpellExpired};

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, Default)]
pub enum SlotCount {
    #[default]
    None,
    Unlimited,
    FixedAmount(usize),
}

/// A descriptor for a
#[derive(Component, Debug, Clone, Serialize, Deserialize, JsonSchema)]
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

#[derive(Component)]
pub struct LifetimeStart(f64);

/// A spell component instantiated
#[derive(Bundle)]
pub struct LiveSpell {
    pub descriptor: SpellComponentDescriptor,
    pub lifetime_start: LifetimeStart,
    pub colider: Collider,
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
        time: Res<Time<Virtual>>,
        descriptor: SpellComponentDescriptor,
    ) -> Self {
        let diameter = 0.1;
        Self {
            name: Name::new(descriptor.friendly_name.clone()),
            lifetime_start: LifetimeStart(time.elapsed_secs_wrapped_f64()),
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
        }
    }
}

pub fn expire_dead_spells(
    mut writer: MessageWriter<ScriptCallbackEvent>,
    time: Res<Time<Virtual>>,
    allocator: ResMut<AppReflectAllocator>,
    spells: Query<(Entity, Ref<SpellComponentDescriptor>, Ref<LifetimeStart>)>,
) {
    let current_time = time.elapsed_secs_wrapped_f64();

    for (entity, spell, lifetime_start) in spells.iter() {
        let diff = (lifetime_start.0 - current_time).abs() * 1000.;
        if diff <= spell.lifetime_milliseconds as f64 {
            continue;
        }

        if let Some(controller) = &spell.script_controller_handle {
            let entity = ReflectReference::new_allocated(entity, &mut allocator.write());
            writer.write(ScriptCallbackEvent::new_for_static_script(
                OnSpellExpired,
                vec![ScriptValue::Reference(entity)],
                controller.clone(),
            ));
        } else {
            log::error!(
                "spell expired with missing script controller: {}",
                spell.friendly_name
            );
        }
    }
}
