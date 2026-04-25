use std::{path::PathBuf, sync::Arc, u32};

use bevy::{
    app::{Plugin, PluginGroup, Startup, Update},
    asset::AssetApp,
    ecs::{
        entity::{Entity, EntityIndex},
        schedule::IntoScheduleConfigs,
    },
    state::condition::in_state,
};
use bevy_mod_scripting::{
    BMSPlugin,
    core::{
        ConfigureScriptPlugin, ScriptingSystemSet,
        script::{ContextKey, ContextKeySelector, ContextPolicy, ContextRule},
    },
    ladfile::plugin::ScriptingFilesGenerationPlugin,
    lua::LuaScriptingPlugin,
    prelude::event_handler,
    script::ScriptAttachment,
};

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
    system_sets::GameSystemSets,
};

pub mod assets;
pub mod bindings;
pub mod loaded_script_descriptors;
pub mod script_descriptor;
pub mod script_descriptor_asset_loader;
pub mod systems;

/// Sets up all interactions between the engine and scripts via BMS
pub struct ScriptLoaderPlugin;

/// Context policy which attaches:
/// - entity attached scripts to their own isolated per-script-per-entity environment
/// - static scripts to their own isolated per-script environment
/// - "spell.lua" ending spell scripts to a globally shared environment
#[derive(Debug)]
pub struct SharedSpellScriptRule;

const SPELL_FILE_EXTENSION: &str = "spell.lua";
/// The entity we use as an attachment point for spell scripts
/// It doesn't actually exist
const RESERVED_SPELL_ENTITY: Entity =
    Entity::from_raw_u32(u32::MAX - 1).expect("Invalid reserved entity");

impl ContextKeySelector for SharedSpellScriptRule {
    fn select(
        &self,
        context_key: &ScriptAttachment,
    ) -> Option<bevy_mod_scripting::core::script::ContextKey> {
        if let ScriptAttachment::StaticScript(handle) = context_key {
            let path = handle.path().expect("script assets are missing paths");
            path.get_full_extension()
                .is_some_and(|e| e == SPELL_FILE_EXTENSION)
                .then_some(ContextKey {
                    entity: Some(RESERVED_SPELL_ENTITY),
                    script: None,
                })
        } else {
            None
        }
    }
}

impl Plugin for ScriptLoaderPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        // Bevy Mod Scripting Framework
        app.add_plugins(
            BMSPlugin
                .set::<LuaScriptingPlugin>(LuaScriptingPlugin::default().set_context_policy(
                    ContextPolicy {
                        priorities: vec![
                            Arc::new(ContextRule::EntityScript),
                            Arc::new(SharedSpellScriptRule),
                            Arc::new(ContextRule::Script),
                        ],
                    },
                ))
                .set::<ScriptingFilesGenerationPlugin>(ScriptingFilesGenerationPlugin::new(
                    true, // enabled, you can use a compilation feature to disable this here
                    PathBuf::from("assets").join("definitions"),
                    Some(PathBuf::from("bindings.lad.json")), // do also save the ladfile itself
                    "Arcane Assembly LUA Interface",
                    true,
                    true,
                )),
        );

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
                    // TODO: use a one time schedule for initialization rather than doing game state checks every update
                    (activate_core_scripts).run_if(in_state(GameState::CoreScriptsLoading)),
                    (load_external_dependencies_in_mods)
                        .chain()
                        .run_if(in_state(GameState::ModDependencyResolution)),
                    (
                        dispaptch_on_update.in_set(GameSystemSets::UpdateDispatch),
                        dispatch_on_player_input.in_set(GameSystemSets::PlayerInputDispatch),
                    )
                        .run_if(in_state(GameState::Running)),
                ),
            )
            .add_systems(
                Update,
                (
                    event_handler::<OnUpdate, LuaScriptingPlugin>
                        .in_set(GameSystemSets::UpdateHandling),
                    event_handler::<OnPlayerInput, LuaScriptingPlugin>
                        .in_set(GameSystemSets::PlayerInputHandling),
                )
                    .run_if(in_state(GameState::Running)),
            );
    }
}
