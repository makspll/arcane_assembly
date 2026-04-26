use crate::{
    mods::{
        mod_descriptor_asset::{ModDescriptor, ModDescriptorAsset},
        systems::{asset_root_path, recurse_dirs},
    },
    spells::{spell, spell_component_asset::SpellComponentAsset},
};
use bevy::{
    asset::{AssetLoader, AssetPath},
    ecs::error::BevyError,
    log::error,
    platform::collections::HashMap,
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

        let mut spell_component_asset_handles =
            HashMap::with_capacity(descriptor.spell_components.len());

        let spell_directory = load_context
            .path()
            .parent()
            .expect("bad path")
            .resolve("spells")
            .expect("invalid parse");
        for spell_component_descriptor in &descriptor.spell_components {
            let script_path = match spell_directory
                .resolve(&format!("{}.lua", spell_component_descriptor.identifier))
            {
                Ok(path) => path,
                Err(err) => {
                    error!(
                        "Invalid script component identifier, could not build path. Mod: {}, component identifier: {}. {err}",
                        descriptor.name, spell_component_descriptor.identifier
                    );
                    continue;
                }
            };
            let spell_component_asset = load_context
                .labeled_asset_scope::<SpellComponentAsset, ()>(
                    format!("spell_component:{}", spell_component_descriptor.identifier),
                    |a| {
                        let script = a.load(script_path);

                        Ok(SpellComponentAsset {
                            descriptor: spell_component_descriptor.clone(),
                            script,
                        })
                    },
                )
                .expect("infallible");

            spell_component_asset_handles.insert(
                spell_component_descriptor.identifier.to_owned(),
                spell_component_asset,
            );
        }

        Ok(ModDescriptorAsset {
            descriptor,
            script,
            spell_component_asset_handles,
        })
    }

    fn extensions(&self) -> &[&str] {
        &["mod.json"]
    }
}
