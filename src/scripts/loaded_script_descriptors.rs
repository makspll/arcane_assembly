use bevy::{
    asset::{AssetEvent, Assets, Handle},
    ecs::{
        message::MessageReader,
        resource::Resource,
        system::{Res, ResMut},
    },
    platform::collections::HashMap,
};

use crate::{
    scripts::script_descriptor::ScriptDescriptor, spells::spell::SpellComponentDescriptor,
};

/// Exists to keep asset handles alive
#[derive(Resource, Default)]
pub struct LoadedScriptDescriptors {
    pub descriptors: Vec<Handle<ScriptDescriptor>>,
}

impl LoadedScriptDescriptors {
    pub fn new(descriptors: Vec<Handle<ScriptDescriptor>>) -> Self {
        Self { descriptors }
    }

    /// Finds the descriptor corresponding to a named mod, if it is loaded
    pub fn get_mod_by_name<'a>(
        &self,
        name: &str,
        assets: &'a Assets<ScriptDescriptor>,
    ) -> Option<(&'a ScriptDescriptor, Handle<ScriptDescriptor>)> {
        self.descriptors.iter().find_map(|handle| {
            assets
                .get(handle)
                .filter(|descriptr| descriptr.name == name)
                .map(|descriptor| (descriptor, handle.clone()))
        })
    }

    /// Finds the descriptor corresponding to a named mod, if it is loaded
    pub fn get_mod_spell_by_name<'a>(
        &self,
        name: &str,
        spell: &str,
        assets: &'a Assets<ScriptDescriptor>,
    ) -> Option<&'a SpellComponentDescriptor> {
        self.descriptors.iter().find_map(|handle| {
            assets
                .get(handle)
                .filter(|descriptor| descriptor.name == name)
                .and_then(|descriptor| {
                    descriptor
                        .spell_components
                        .iter()
                        .find(|d| d.friendly_name == spell)
                })
        })
    }
}
