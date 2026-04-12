use bevy::{
    asset::{Asset, Handle, LoadedUntypedAsset},
    reflect::TypePath,
};
use bevy_mod_scripting::asset::ScriptAsset;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct ScriptDescriptor {
    pub name: String,
    pub kind: ScriptKind,
    pub description: String,
    pub version: String,
    pub attach_kind: AttachKind,
    pub dependant_on_lua_scripts: Vec<String>,
    #[serde(skip_deserializing, skip_serializing, default)]
    pub script: Handle<ScriptAsset>,
    #[serde(skip_deserializing, skip_serializing, default)]
    pub assets: Vec<Handle<LoadedUntypedAsset>>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub enum ScriptKind {
    Core,
    User,
}

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub enum AttachKind {
    Player,
    Static,
}
