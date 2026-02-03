use std::path::Path;

use bevy::{
    asset::{AssetPlugin, AssetServer, Assets, io::file::FileAssetReader},
    ecs::{
        message::MessageWriter,
        system::{Commands, Res, ResMut},
    },
    state::state::NextState,
};
use bevy_console::PrintConsoleLine;
use bevy_mod_scripting::{
    core::pipeline::ScriptPipelineState, lua::LuaScriptingPlugin, prelude::AttachScript,
};

use crate::{
    scripts::{
        loaded_script_descriptors::LoadedScriptDescriptors,
        script_descriptor::{ScriptDescriptor, ScriptKind},
    },
    state::GameState,
};

/// Transitions the game state once all currently loading scripts are finished processing
pub fn activate_core_scripts(
    pipeline_state: ScriptPipelineState<LuaScriptingPlugin>,
    mut next_state: ResMut<NextState<GameState>>,
    mut console_message: MessageWriter<PrintConsoleLine>,
    loaded_script_descriptors: ResMut<LoadedScriptDescriptors>,
    script_descriptor_assets: Res<Assets<ScriptDescriptor>>,
    mut commands: Commands,
) {
    let (_, loaded, total) = pipeline_state.progress();

    // first frame of loading
    if total == 0 {
        console_message.write(PrintConsoleLine::new(String::from(
            "Initiating Core Scripts",
        )));
        for descriptor in &loaded_script_descriptors.descriptors {
            if let Some(descriptor_asset) = script_descriptor_assets.get(descriptor)
                && descriptor_asset.kind == ScriptKind::Core
            {
                commands.queue(AttachScript::<LuaScriptingPlugin>::new(
                    bevy_mod_scripting::script::ScriptAttachment::StaticScript(
                        descriptor_asset.script.clone(),
                    ),
                ));
            }
        }
    }

    if pipeline_state.processing_batch_completed() && loaded > 0 {
        console_message.write(PrintConsoleLine::new(format!("Loaded {total} mods")));
        next_state.set(GameState::Running)
    }
}

/// Initializes script loading expecting mod descriptors, and only loading the lua scripts that correspond to each descriptor
pub fn init_load_of_all_script_mods(
    mut server: ResMut<AssetServer>,
    mut loaded_script_descriptors: ResMut<LoadedScriptDescriptors>,
) {
    // TODO: when bevy supports loading a specific type of extension / asset type from a folder, use that instead
    // I imagine this would cause issues on non standard platforms
    let asset_root_path = FileAssetReader::get_base_path()
        .join(AssetPlugin::default().file_path + std::path::MAIN_SEPARATOR_STR);

    let asset_root_path_str = asset_root_path.to_string_lossy().to_string();

    let server_ref = &mut server;
    recurse_dirs(&asset_root_path, "mod.json", &mut |abs_path| {
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
    target_ext: &str,
    processor: &mut impl FnMut(&Path),
) -> std::io::Result<()> {
    let dir = std::fs::read_dir(root)?;
    for dir in dir {
        let dir = dir?;
        if dir.path().to_string_lossy().ends_with(target_ext) {
            processor(&dir.path());
        }
        if dir.file_type()?.is_dir() {
            recurse_dirs(&dir.path(), target_ext, processor)?;
        }
    }
    Ok(())
}
