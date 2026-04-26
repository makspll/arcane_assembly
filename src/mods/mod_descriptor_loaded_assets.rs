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
    mods::mod_descriptor_asset::ModDescriptorAsset,
    spells::spell::{SpellComponentDescriptor, SpellComponentDescriptorHandle},
};

/// Exists to keep asset handles alive
#[derive(Resource, Default)]
pub struct ModDescriptorLoadedAssets {
    pub descriptors: Vec<Handle<ModDescriptorAsset>>,
}

impl ModDescriptorLoadedAssets {
    pub fn new(descriptors: Vec<Handle<ModDescriptorAsset>>) -> Self {
        Self { descriptors }
    }

    /// Finds the descriptor corresponding to a named mod, if it is loaded
    pub fn get_mod_by_name<'a>(
        &self,
        name: &str,
        assets: &'a Assets<ModDescriptorAsset>,
    ) -> Option<(&'a ModDescriptorAsset, Handle<ModDescriptorAsset>)> {
        self.descriptors.iter().find_map(|handle| {
            assets
                .get(handle)
                .filter(|descriptr| descriptr.name == name)
                .map(|descriptor| (descriptor, handle.clone()))
        })
    }

    /// Finds the descriptor corresponding to a named mod, if it is loaded
    pub fn get_spell_component_by_name<'a>(
        &self,
        name: &str,
        spell_component: &str,
        assets: &'a Assets<ModDescriptorAsset>,
    ) -> Option<SpellComponentDescriptorHandle> {
        self.descriptors
            .iter()
            .find_map(|handle| {
                assets
                    .get(handle)
                    .filter(|descriptor| descriptor.name == name)
                    .and_then(|descriptor| {
                        descriptor
                            .spell_components
                            .iter()
                            .find(|d| d.friendly_name == spell_component)
                    })
            })
            .cloned()
            .map(SpellComponentDescriptorHandle)
    }
}
