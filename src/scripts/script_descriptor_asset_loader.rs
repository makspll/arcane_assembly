use crate::scripts::{
    script_descriptor::ScriptDescriptor,
    systems::{asset_root_path, recurse_dirs},
};
use bevy::{
    asset::{AssetLoader, AssetPath},
    ecs::error::BevyError,
    reflect::TypePath,
};

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
        let script_assets_path = asset_path.parent().unwrap().join("assets");
        let absolute_script_assets_path = asset_root_path().join(script_assets_path);

        if absolute_script_assets_path.is_dir() {
            recurse_dirs(&absolute_script_assets_path, None, &mut |f| {
                let path = f.to_owned();
                asset
                    .assets
                    .push(load_context.loader().with_unknown_type().load(
                        AssetPath::from_path_buf(
                            path.strip_prefix(asset_root_path()).unwrap().to_owned(),
                        ),
                    ));
            })
            .unwrap();
        }

        asset.script = load_context.load(script_path);

        Ok(asset)
    }

    fn extensions(&self) -> &[&str] {
        &["mod.json"]
    }
}
