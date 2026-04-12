use bevy::{app::Plugin, asset::AssetApp};
use bevy_mod_scripting::{prelude::ScriptComponent, script::ScriptAttachment};

// use crate::sprite::aseprite::AsepriteWrapperTransformer;

pub mod aseprite;

pub struct SpritesPlugin;

impl Plugin for SpritesPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        // app.register_asset_processor(AsepriteWrapperTransformer);
        // ScriptComponent
    }
}
