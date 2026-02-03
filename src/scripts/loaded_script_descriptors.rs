use bevy::{asset::Handle, ecs::resource::Resource};

use crate::scripts::script_descriptor::ScriptDescriptor;

#[derive(Resource, Default)]
pub struct LoadedScriptDescriptors {
    pub descriptors: Vec<Handle<ScriptDescriptor>>,
}
