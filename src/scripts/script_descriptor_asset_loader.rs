use bevy::{asset::AssetLoader, ecs::error::BevyError, reflect::TypePath};

use crate::scripts::script_descriptor::ScriptDescriptor;

#[derive(TypePath)]
/// Loader which produces [`ScriptDescriptor`] assets, with the underlying scripts as asset dependencies.
/// This allows us to plug in metadata, and control script loading via the expected mod structure.
pub struct ScriptDescriptorAssetLoader;

impl AssetLoader for ScriptDescriptorAssetLoader {
    type Asset = ScriptDescriptor;
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

        let mut asset: ScriptDescriptor =
            serde_json::de::from_slice(&buf).map_err(BevyError::from)?;

        let asset_path = load_context.path().path();
        let script_path = asset_path.with_extension("").with_extension("lua");

        asset.script = load_context.load(script_path);

        Ok(asset)
    }

    fn extensions(&self) -> &[&str] {
        &["mod.json"]
    }
}
