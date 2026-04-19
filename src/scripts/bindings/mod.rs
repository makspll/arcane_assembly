use crate::{
    scripts::{
        assets::load_untyped_asset_for_script_descriptor,
        bindings::audio::register_audio_global_functions,
        loaded_script_descriptors::LoadedScriptDescriptors, script_descriptor::ScriptDescriptor,
    },
    sprite::aseprite::{AsepriteHandle, set_aseprite_animation_on_entity},
};
use bevy::{
    app::{App, Plugin},
    asset::{AssetServer, Assets, Handle, LoadedUntypedAsset},
    ecs::{entity::Entity, world::Mut},
    prelude::World,
};
use bevy_mod_scripting::{
    bindings::{FunctionCallContext, InteropError, V, WorldExtensions},
    script_bindings,
};

mod audio;

#[script_bindings(name = "global_functions", remote, unregistered)]
impl World {
    /// Loads asset from the given mod, relative to the directory containing ".mod.json" files
    /// Arguments:
    /// * `mod_name` - the name of the mod from which to load the asset
    /// * `path` - the relative path from the root of the mod to the asset
    /// Returns:
    /// * `untyped_handle` - the handle to this asset, which when the asset is fully loaded will be accessible
    fn load_asset_from_mod(
        ctxt: FunctionCallContext,
        mod_name: String,
        path: String,
    ) -> Result<Option<V<Handle<LoadedUntypedAsset>>>, InteropError> {
        let world = ctxt.world()?;
        let o =
            world.with_resource(|script_descriptor_assets: &Assets<ScriptDescriptor>| {
                world.with_resource(|loaded_script_descriptors: &LoadedScriptDescriptors| {
                    world.with_resource_mut(|asset_server: Mut<AssetServer>| {
                        load_untyped_asset_for_script_descriptor(
                            &mod_name,
                            &path,
                            script_descriptor_assets,
                            loaded_script_descriptors,
                            asset_server,
                        )
                        .unwrap()
                    })
                })
            })???;
        Ok(o.map(V::from))
    }
}

#[script_bindings(name = "entity_functions", remote)]
impl Entity {
    fn set_aseprite_animation(
        ctxt: FunctionCallContext,
        entity: V<Entity>,
        spritesheet: AsepriteHandle,
        animation_tag: String,
        flip_sprite: Option<bool>,
    ) -> Result<(), InteropError> {
        let world = ctxt.world()?;
        world.with_world_mut_access(|w| {
            set_aseprite_animation_on_entity(
                w.commands(),
                spritesheet.0,
                entity.0,
                &animation_tag,
                flip_sprite.unwrap_or(false),
            );
            w.flush();
        })?;
        Ok(())
    }
}

pub struct ScriptBindingsPlugin;

impl Plugin for ScriptBindingsPlugin {
    fn build(&self, app: &mut App) {
        let world = app.world_mut();
        register_global_functions(world);
        register_entity_functions(world);
        register_audio_global_functions(world);
    }
}
