use bevy::{
    app::{Plugin, Startup, Update},
    asset::AssetApp,
    ecs::schedule::IntoScheduleConfigs,
    state::condition::in_state,
};

use crate::{
    scripts::{
        loaded_script_descriptors::LoadedScriptDescriptors,
        script_descriptor::ScriptDescriptor,
        script_descriptor_asset_loader::ScriptDescriptorAssetLoader,
        systems::{activate_core_scripts, init_load_of_all_script_mods},
    },
    state::GameState,
};

pub mod bindings;
pub mod loaded_script_descriptors;
pub mod script_descriptor;
pub mod script_descriptor_asset_loader;
pub mod systems;

/// Sets up all interactions between the engine and scripts via BMS
pub struct ScriptLoaderPlugin;

impl Plugin for ScriptLoaderPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_asset::<ScriptDescriptor>()
            .register_asset_loader(ScriptDescriptorAssetLoader)
            .init_resource::<LoadedScriptDescriptors>()
            .add_systems(Startup, init_load_of_all_script_mods)
            .add_systems(
                Update,
                activate_core_scripts.run_if(in_state(GameState::CoreScriptsLoading)),
            );
    }
}
