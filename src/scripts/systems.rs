use bevy::{
    asset::{self, AssetId, AssetPlugin, AssetServer, Assets, Handle, io::file::FileAssetReader},
    ecs::{
        entity::Entity,
        message::MessageWriter,
        query::With,
        schedule::And,
        system::{Commands, Local, Query, Res, ResMut},
        world::Ref,
    },
    input::{
        ButtonInput,
        keyboard::{Key, KeyCode},
    },
    log::{self, info},
    platform::collections::{HashMap, HashSet},
    state::state::NextState,
    time::{Real, Time},
};
use bevy_mod_scripting::{
    asset::ScriptAsset,
    core::{callback_labels, event::ScriptAttachedEvent, pipeline::ScriptPipelineState},
    lua::LuaScriptingPlugin,
    prelude::{AttachScript, ScriptCallbackEvent, ScriptComponent, ScriptValue},
    script::ScriptAttachment,
};
use schemars::schema_for;
use std::{
    collections::VecDeque,
    path::{Path, PathBuf},
};

use crate::{
    character::controllable_character::Player,
    input::PlayerInput,
    scripts::{
        loaded_script_descriptors::LoadedScriptDescriptors,
        script_descriptor::{AttachKind, ScriptDescriptor, ScriptKind},
    },
    state::GameState,
};

/// Transitions the game state once all currently loading scripts are finished processing
pub fn activate_core_scripts(
    mut started: Local<bool>,
    pipeline_state: ScriptPipelineState<LuaScriptingPlugin>,
    mut next_state: ResMut<NextState<GameState>>,
    loaded_script_descriptors: ResMut<LoadedScriptDescriptors>,
    script_descriptor_assets: Res<Assets<ScriptDescriptor>>,
    player: Query<Entity, With<Player>>,
    mut commands: Commands,
) {
    let (_, loaded, total) = pipeline_state.progress();

    // first frame of loading
    if total == 0 && !*started {
        *started = true;
        info!("Initializing core scripts");
        for descriptor in &loaded_script_descriptors.descriptors {
            if let Some(descriptor_asset) = script_descriptor_assets.get(descriptor)
                && descriptor_asset.kind == ScriptKind::Core
            {
                let handle = descriptor_asset.script.clone();
                match descriptor_asset.attach_kind {
                    AttachKind::Static => {
                        commands.queue(AttachScript::<LuaScriptingPlugin>::new(
                            bevy_mod_scripting::script::ScriptAttachment::StaticScript(
                                handle.clone(),
                            ),
                        ));
                    }
                    AttachKind::Player => {
                        // cuz these functions need static lifetimes, rust needs an explicit double clone
                        let handle_clone = handle.clone();
                        commands
                            .entity(player.single().expect("player entity not spawned"))
                            .entry::<ScriptComponent>()
                            .or_insert_with(move || ScriptComponent(vec![handle_clone]))
                            .and_modify(move |mut c| c.0.push(handle));
                    }
                }
            }
        }
    }

    if pipeline_state.processing_batch_completed() && loaded > 1 {
        info!("Loaded {total} mods");
        next_state.set(GameState::ModDependencyResolution)
    }
}

pub fn sync_dev_schema() {
    let schema = serde_json::to_string_pretty(&schema_for!(ScriptDescriptor))
        .expect("Failed to serialize mod schema");
    let path = asset_root_path()
        .join("definitions")
        .join("mod_descriptor_schema.json");
    std::fs::write(path, schema).expect("Failed to update mod schema");
}

pub fn asset_root_path() -> PathBuf {
    FileAssetReader::get_base_path()
        .join(AssetPlugin::default().file_path + std::path::MAIN_SEPARATOR_STR)
}
/// Initializes script loading expecting mod descriptors, and only loading the lua scripts that correspond to each descriptor
pub fn init_load_of_all_script_mods(
    mut server: ResMut<AssetServer>,
    mut loaded_script_descriptors: ResMut<LoadedScriptDescriptors>,
) {
    // TODO: when bevy supports loading a specific type of extension / asset type from a folder, use that instead
    // I imagine this would cause issues on non standard platforms
    let asset_root_path = asset_root_path();

    let asset_root_path_str = asset_root_path.to_string_lossy().to_string();

    let server_ref = &mut server;
    recurse_dirs(&asset_root_path, Some("mod.json"), &mut |abs_path| {
        let asset_relative_path = abs_path.to_string_lossy();
        let asset_relative_path = asset_relative_path
            .strip_prefix(&asset_root_path_str)
            .unwrap_or_else(|| {
                panic!("path {abs_path:?} did not start with the asset path {asset_root_path_str}",)
            })
            .to_string();

        loaded_script_descriptors
            .descriptors
            .push(server_ref.load(asset_relative_path));
    })
    .expect("failed to read script assets");
}

/// Completes loading of mods, by resolving any pointers to external mods as handles etc.
///
/// Ideally we should also re-do this if new assets are added, but that's rare enough it's probably fine, downstream changes to the script will re-load the script itself.
pub fn load_external_dependencies_in_mods(
    mut commands: Commands,
    loaded_script_descriptors: Res<LoadedScriptDescriptors>,
    mut descriptors: ResMut<Assets<ScriptDescriptor>>,
    mut next_state: ResMut<NextState<GameState>>,
    asset_server: Res<AssetServer>,
) {
    let mut script_resolutions: HashMap<(AssetId<ScriptDescriptor>, usize), Handle<ScriptAsset>> =
        Default::default();
    let mut static_scripts_to_attach: HashSet<Handle<ScriptAsset>> = Default::default();

    for (asset_id, asset) in descriptors.iter() {
        for (idx, spell_component) in asset.spell_components.iter().enumerate() {
            let resolution = match spell_component
                .script_controller_path
                .asset_path(&loaded_script_descriptors, &descriptors)
            {
                Ok(resolved) => asset_server.load(resolved),
                Err(err) => {
                    log::error!(
                        "Failed to resolve script dependency in mod: '{}', on spell_component_controller: '{}': {err}",
                        asset.name,
                        spell_component.script_controller_path
                    );
                    continue;
                }
            };
            script_resolutions.insert((asset_id, idx), resolution.clone());
            static_scripts_to_attach.insert(resolution);
        }
    }

    // apply resolutions
    for ((asset_id, spell_component_idx), resolved_script) in script_resolutions {
        let asset = descriptors
            .get_mut(asset_id)
            .expect("invariant broken: previously resolved asset missing");
        asset.spell_components[spell_component_idx].script_controller_handle =
            Some(resolved_script);
    }

    for script in static_scripts_to_attach.drain() {
        commands.queue(AttachScript::<LuaScriptingPlugin>::new(
            ScriptAttachment::StaticScript(script),
        ));
    }

    next_state.set(GameState::Running);
}

/// Recurse from the given root directory, running the processor on every found file of the given extension
pub fn recurse_dirs(
    root: &Path,
    target_ext: Option<&str>,
    processor: &mut impl FnMut(&Path),
) -> std::io::Result<()> {
    let dir = std::fs::read_dir(root)?;
    for dir in dir {
        let dir = dir?;

        if dir.file_type()?.is_dir() {
            recurse_dirs(&dir.path(), target_ext, processor)?;
        } else if target_ext
            .is_none_or(|target_ext| dir.path().to_string_lossy().ends_with(target_ext))
        {
            processor(&dir.path());
        }
    }
    Ok(())
}

callback_labels!(
    OnUpdate => "on_update",
    OnPlayerInput => "on_player_input",
);

// should we use virtual time here? if we decide to start pausing stuff re-visit
pub fn dispaptch_on_update(time: Res<Time<Real>>, mut writer: MessageWriter<ScriptCallbackEvent>) {
    let dt = ScriptValue::Float(time.delta_secs_f64());
    let time_seconds = ScriptValue::Float(time.elapsed_secs_f64());
    writer.write(ScriptCallbackEvent::new_for_all_contexts(
        OnUpdate,
        vec![dt, time_seconds],
    ));
}

pub fn dispatch_on_player_input(
    time: Res<Time<Real>>,
    mut any_inputs_last_frame: Local<bool>,
    mut writer: MessageWriter<ScriptCallbackEvent>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_scripts: Query<(Entity, Ref<ScriptComponent>), (With<Player>, With<ScriptComponent>)>,
) {
    let inputs = keyboard_input
        .get_pressed()
        .map(PlayerInput::from)
        .filter(|i| !matches!(i, PlayerInput::Unknown))
        .map(ScriptValue::from)
        .collect::<VecDeque<_>>();

    // only trigger the first time no inputs are present and any time there are buttons pressed
    if inputs.is_empty() {
        if !*any_inputs_last_frame {
            return;
        }
        *any_inputs_last_frame = false;
    } else {
        *any_inputs_last_frame = true;
    }
    let time_seconds = ScriptValue::Float(time.elapsed_secs_f64());
    for (entity, scripts) in player_scripts {
        let events = scripts
            .0
            .iter()
            .map(|script| {
                ScriptCallbackEvent::new_for_script_entity(
                    OnPlayerInput,
                    vec![ScriptValue::List(inputs.clone()), time_seconds.clone()],
                    script.clone(),
                    entity,
                )
            })
            .collect::<Vec<_>>();
        writer.write_batch(events);
    }
}
