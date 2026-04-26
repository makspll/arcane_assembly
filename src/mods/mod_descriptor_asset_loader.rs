use crate::{
    mods::{
        mod_descriptor_asset::{ModDescriptor, ModDescriptorAsset},
        systems::{asset_root_path, recurse_dirs},
    },
    spells::spell,
};
use bevy::{
    asset::{AssetLoader, AssetPath},
    ecs::error::BevyError,
    reflect::TypePath,
};

#[derive(TypePath)]
/// Loader which produces [`ScriptDescriptor`] assets, with the underlying scripts as asset dependencies.
/// This allows us to plug in metadata, and control script loading via the expected mod structure.
pub struct ModDescriptorAssetLoader;

impl AssetLoader for ModDescriptorAssetLoader {
    type Asset = ModDescriptorAsset;
    type Settings = ();

    type Error = BevyError;

    async fn load(
        &self,
        reader: &mut dyn bevy::asset::io::Reader,
        _settings: &Self::Settings,
        load_context: &mut bevy::asset::LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut buf = Vec::new();

        reader.read_to_end(&mut buf).await?;

        let descriptor: ModDescriptor =
            serde_json::de::from_slice(&buf).map_err(BevyError::from)?;

        let asset_path = load_context.path().path();
        let script_path = asset_path.with_extension("").with_extension("lua");

        let script = load_context.load(script_path);

        Ok(ModDescriptorAsset { descriptor, script })
    }

    fn extensions(&self) -> &[&str] {
        &["mod.json"]
    }
}
