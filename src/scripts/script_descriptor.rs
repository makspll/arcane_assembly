use bevy::{
    asset::{Asset, Handle},
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
    pub dependant_on_lua_scripts: Vec<String>,
    #[serde(skip_deserializing, skip_serializing, default)]
    pub script: Handle<ScriptAsset>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub enum ScriptKind {
    Core,
    User,
}
