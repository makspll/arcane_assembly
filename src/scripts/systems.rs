use bevy::{
    asset::{AssetPlugin, AssetServer, Assets, io::file::FileAssetReader},
    ecs::{
        entity::Entity,
        message::MessageWriter,
        query::With,
        system::{Commands, Query, Res, ResMut},
    },
    input::{ButtonInput, keyboard::Key},
    log::info,
    state::state::NextState,
};
use bevy_mod_scripting::{
    core::{callback_labels, pipeline::ScriptPipelineState},
    lua::LuaScriptingPlugin,
    prelude::{AttachScript, ScriptCallbackEvent, ScriptComponent},
};
use std::path::{Path, PathBuf};

use crate::{
    character::controllable_character::Player,
    scripts::{
        loaded_script_descriptors::LoadedScriptDescriptors,
        script_descriptor::{AttachKind, ScriptDescriptor, ScriptKind},
    },
    state::GameState,
};

/// Transitions the game state once all currently loading scripts are finished processing
pub fn activate_core_scripts(
    pipeline_state: ScriptPipelineState<LuaScriptingPlugin>,
    mut next_state: ResMut<NextState<GameState>>,
    loaded_script_descriptors: ResMut<LoadedScriptDescriptors>,
    script_descriptor_assets: Res<Assets<ScriptDescriptor>>,
    player: Query<Entity, With<Player>>,
    mut commands: Commands,
) {
    let (_, loaded, total) = pipeline_state.progress();

    // first frame of loading
    if total == 0 {
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
                        commands
                            .entity(player.single().expect("player entity not spawned"))
                            .entry::<ScriptComponent>()
                            .and_modify(move |mut c| c.0.push(handle.clone()));
                    }
                }
            }
        }
    }

    if pipeline_state.processing_batch_completed() && loaded > 1 {
        info!("Loaded {total} mods");
        next_state.set(GameState::Running)
    }
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

pub fn dispaptch_on_update(mut writer: MessageWriter<ScriptCallbackEvent>) {
    writer.write(ScriptCallbackEvent::new_for_all_contexts(OnUpdate, vec![]));
}

pub fn dispatch_on_input(
    mut writer: MessageWriter<ScriptCallbackEvent>,
    keyboard_input: Res<ButtonInput<Key>>,
) {
    let pressed = keyboard_input.get_just_pressed().collect::<Vec<_>>();

    // .write(ScriptCallbackEvent::new_for_all_contexts(OnInput, vec![]));
}
