use bevy::{
    asset::{AssetServer, Assets, Handle, LoadedUntypedAsset, ParseAssetPathError, UntypedHandle},
    ecs::world::Mut,
    log::info,
};

use crate::scripts::{
    loaded_script_descriptors::{self, LoadedScriptDescriptors},
    script_descriptor::ScriptDescriptor,
};

pub fn load_untyped_asset_for_script_descriptor(
    mod_name: &str,
    path: &str,
    script_descriptor_assets: &Assets<ScriptDescriptor>,
    loaded_script_descriptors: &LoadedScriptDescriptors,
    asset_server: Mut<AssetServer>,
) -> Result<Option<Handle<LoadedUntypedAsset>>, ParseAssetPathError> {
    let opt_descriptor_and_handle =
        loaded_script_descriptors.get_mod_by_name(mod_name, script_descriptor_assets);

    match opt_descriptor_and_handle {
        // handles which don't stem directly from a `AssetServer::load` don't have paths
        // ours should
        Some((descriptor, handle)) if handle.path().is_some() => {
            let mod_asset_path = handle
                .path()
                .unwrap()
                .parent()
                .unwrap()
                .resolve("assets")
                .unwrap();
            let mod_relative_asset_path = mod_asset_path.resolve(path)?;

            info!("Loading asset for mod: {mod_name}, from: '{mod_relative_asset_path}'");

            if let Some(cached) = descriptor
                .assets
                .iter()
                .find(|h| h.path() == Some(&mod_relative_asset_path))
            {
                return Ok(Some(cached.clone()));
            }
            // TODO: cache
            Ok(Some(asset_server.load_untyped(mod_relative_asset_path)))
        }
        _ => Ok(None),
    }
}
