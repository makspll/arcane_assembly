use std::{
    error::Error,
    fmt::Display,
    path::{Path, PathBuf},
    sync::Arc,
};

use bevy::{
    asset::{Asset, AssetPath, Assets, Handle, LoadedUntypedAsset},
    reflect::{Reflect, TypePath},
};
use bevy_mod_scripting::asset::ScriptAsset;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    mods::mod_descriptor_loaded_assets::ModDescriptorLoadedAssets,
    spells::spell::SpellComponentDescriptor,
};

#[derive(Asset, TypePath)]
pub struct ModDescriptorAsset {
    pub descriptor: ModDescriptor,
    pub script: Handle<ScriptAsset>,
}

/// Describes the contents of a mod
#[derive(Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ModDescriptor {
    pub name: String,
    pub kind: ScriptKind,
    pub description: String,
    pub version: String,
    pub attach_kind: AttachKind,
    pub dependant_on_lua_scripts: Vec<String>,
    pub spell_components: Vec<Arc<SpellComponentDescriptor>>,
}

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
pub enum ScriptKind {
    Core,
    User,
}

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
pub enum AttachKind {
    Player,
    Static,
}

/// A [`PathBuf`] but pointing to a specific mod
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    JsonSchema,
    Reflect,
)]
pub struct ModPathBuf {
    pub mod_name: String,
    pub path: PathBuf,
}

impl Display for ModPathBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.mod_name)?;
        f.write_str(": ")?;
        f.write_str(self.path.to_str().unwrap_or_default())
    }
}

impl ModPathBuf {
    /// Creates a new path pointing to an asset of '<mod>/assets/<path>' structure
    pub fn new_mod_assets_path(
        mod_name: &str,
        path_in_mod_assets_folder: impl Into<PathBuf>,
    ) -> Self {
        Self {
            mod_name: mod_name.to_owned(),
            path: PathBuf::from("assets").join(path_in_mod_assets_folder.into()),
        }
    }

    pub fn asset_path(
        &self,
        loaded_script_descriptors: &ModDescriptorLoadedAssets,
        descriptor_assets: &Assets<ModDescriptorAsset>,
    ) -> Result<AssetPath<'_>, Box<dyn Error>> {
        let (_, root_handle) = loaded_script_descriptors
            .get_mod_by_name(&self.mod_name, descriptor_assets)
            .ok_or_else(|| format!("could not find mod: {}", self.mod_name))?;

        let utf_path = self
            .path
            .to_str()
            .ok_or_else(|| format!("path contained non-utf8 characters: {self:?}"))?;

        let asset_path = root_handle
            .path()
            .ok_or(
                "asset handle for script descriptor without path. Did something load incorrectly ?",
            )?
            .parent()
            .expect("weird path")
            .resolve(utf_path)?;
        Ok(asset_path)
    }
}
