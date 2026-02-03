use bevy::{asset::Assets, ecs::system::Res};
use bevy_console::{ConsoleCommand, clap};

use crate::scripts::{
    loaded_script_descriptors::LoadedScriptDescriptors, script_descriptor::ScriptDescriptor,
};

#[derive(clap::Parser, ConsoleCommand)]
#[command(name = "list_mods")]
pub struct ListModsCmnd {}

pub fn list_mods_cmd(
    mut log: ConsoleCommand<ListModsCmnd>,
    descriptors: Res<LoadedScriptDescriptors>,
    descriptor_assets: Res<Assets<ScriptDescriptor>>,
) {
    if let Some(Ok(_)) = log.take() {
        log.reply(format!(
            "Loaded mods count: {}",
            descriptors.descriptors.len()
        ));
        for descriptor in &descriptors.descriptors {
            match descriptor_assets.get(descriptor) {
                Some(asset) => {
                    log.reply(format!("\t - {}, {}", asset.name, asset.description));
                }
                None => {
                    log.reply("\t - Unloaded Mod");
                }
            }
        }
    }
}
