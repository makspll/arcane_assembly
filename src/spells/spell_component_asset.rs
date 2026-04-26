use bevy::{
    asset::{Asset, Handle},
    reflect::{Reflect, TypePath},
};
use bevy_mod_scripting::asset::ScriptAsset;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Asset, TypePath, Default)]
pub struct SpellComponentAsset {
    pub descriptor: SpellComponentDescriptor,
    pub script: Handle<ScriptAsset>,
}

/// A descriptor for a
#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, Reflect)]
pub struct SpellComponentDescriptor {
    /// A unique (within the mod) spell name. Typically in snake case.
    pub identifier: String,
    /// The postfix to give to every handler for this spell. For example if this name is 'my_spell' then the handler for casting the spell
    /// would have to be called "on_cast_my_spell", and similar for all the other spell callbacks.
    pub handler_label: String,
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

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, Default, Reflect)]
pub enum SlotCount {
    #[default]
    None,
    Unlimited,
    FixedAmount(usize),
}

impl std::fmt::Debug for SpellComponentAsset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpellComponentDescriptor")
            .field("friendly_name", &self.descriptor.identifier)
            .field("handler_label", &self.descriptor.handler_label)
            .finish()
    }
}
