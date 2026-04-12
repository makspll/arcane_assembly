use bevy::{
    asset::{AssetServer, Assets, Handle},
    ecs::resource::Resource,
};

use crate::scripts::script_descriptor::ScriptDescriptor;

/// Exists to keep asset handles alive
#[derive(Resource, Default)]
pub struct LoadedScriptDescriptors {
    pub descriptors: Vec<Handle<ScriptDescriptor>>,
}

impl LoadedScriptDescriptors {
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
}
