use bevy::{asset::Assets, ecs::system::Res};
use bevy_console::{ConsoleCommand, clap};

use crate::mods::{
    mod_descriptor_loaded_assets::ModDescriptorLoadedAssets, mod_descriptor_asset::ModDescriptorAsset,
};

#[derive(clap::Parser, ConsoleCommand)]
#[command(name = "list_mods")]
pub struct ListModsCmnd {}

pub fn list_mods_cmd(
    mut log: ConsoleCommand<ListModsCmnd>,
    descriptors: Res<ModDescriptorLoadedAssets>,
    descriptor_assets: Res<Assets<ModDescriptorAsset>>,
) {
    if let Some(Ok(_)) = log.take() {
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
