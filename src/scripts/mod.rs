use bevy::{
    app::{Plugin, Startup, Update},
    asset::AssetApp,
    ecs::schedule::IntoScheduleConfigs,
    state::condition::in_state,
};
use bevy_mod_scripting::{lua::LuaScriptingPlugin, prelude::event_handler};

use crate::{
    character::spawn_player_entity,
    scripts::{
        loaded_script_descriptors::LoadedScriptDescriptors,
        script_descriptor::ScriptDescriptor,
        script_descriptor_asset_loader::ScriptDescriptorAssetLoader,
        systems::{
            OnPlayerInput, OnUpdate, activate_core_scripts, dispaptch_on_update,
            dispatch_on_player_input, init_load_of_all_script_mods,
            load_external_dependencies_in_mods,
        },
    },
    state::GameState,
};

pub mod assets;
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
            .add_systems(
                Startup,
                init_load_of_all_script_mods.after(spawn_player_entity),
            )
            .add_systems(
                Update,
                (
                    (activate_core_scripts).run_if(in_state(GameState::CoreScriptsLoading)),
                    (load_external_dependencies_in_mods)
                        .chain()
                        .run_if(in_state(GameState::ModDependencyResolution)),
                    (dispaptch_on_update, dispatch_on_player_input)
                        .run_if(in_state(GameState::Running)),
                ),
            )
            .add_systems(
                Update,
                (
                    event_handler::<OnUpdate, LuaScriptingPlugin>,
                    event_handler::<OnPlayerInput, LuaScriptingPlugin>,
                )
                    .after(dispaptch_on_update)
                    .after(dispatch_on_player_input)
                    .run_if(in_state(GameState::Running)),
            );
    }
}
