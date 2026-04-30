use std::any::TypeId;

use crate::mods::{
    mod_descriptor_asset::ModDescriptorAsset,
    mod_descriptor_loaded_assets::ModDescriptorLoadedAssets,
};
use bevy::{
    asset::{
        Asset, AssetServer, Assets, Handle, LoadedUntypedAsset, ParseAssetPathError, StrongHandle,
        UntypedHandle,
    },
    ecs::world::Mut,
    log::info,
    reflect::{FromReflect, PartialReflect, Reflect, TypeRegistry, Typed},
};
use bevy_mod_scripting::{
    IntoScript,
    bindings::{
        ArgMeta, FromScript, GetTypeDependencies, InteropError, IntoScript, TypedThrough, V,
        WorldExtensions,
    },
    display::{WithTypeInfo, WorldAccessGuard},
    prelude::ScriptValue,
};

/// A newtype around [`Handle<T>`], with de-sugaring implemented for script binding code.
///
/// We can use this to convert the [`UntypedHandle`]'s returned from load bindings, to a specific asset at the binding boundary.
#[derive(Clone, Debug, Reflect)]
pub struct ScriptHandleWrapper<T: Asset>(pub Handle<T>);

impl<T: Asset> IntoScript for ScriptHandleWrapper<T> {
    fn into_script(
        self,
        world: bevy_mod_scripting::bindings::WorldGuard,
    ) -> Result<ScriptValue, InteropError> {
        V::new(self.0).into_script(world)
    }
}

impl<T: Asset> From<Handle<T>> for ScriptHandleWrapper<T> {
    fn from(value: Handle<T>) -> Self {
        Self(value)
    }
}

impl<T: Asset> GetTypeDependencies for ScriptHandleWrapper<T> {
    type Underlying = Handle<T>;

    fn register_type_dependencies(registry: &mut TypeRegistry) {
        registry.register::<Handle<T>>();
    }
}

// lua declaration files don't really support generics, and we will always expect an [`UntypedHandle`] so we can just use that instead
impl<T: Asset> TypedThrough for ScriptHandleWrapper<T> {
    fn through_type_info() -> bevy_mod_scripting::bindings::ThroughTypeInfo {
        bevy_mod_scripting::bindings::ThroughTypeInfo::TypeInfo(UntypedHandle::type_info())
    }
}

impl<T: Asset> ArgMeta for ScriptHandleWrapper<T> {}

impl<T: Asset> FromScript for ScriptHandleWrapper<T> {
    type This<'w> = Self;

    fn from_script(
        value: ScriptValue,
        world: WorldAccessGuard<'_>,
    ) -> Result<Self::This<'_>, bevy_mod_scripting::bindings::InteropError>
    where
        Self: Sized,
    {
        // TODO: implement a proper asset loading phase for scripts
        // and make a distinction between loading and loaded assets
        if let Ok(v) = V::<Handle<T>>::from_script(value.clone(), world.clone()) {
            // buggy reflection :C
            if v.0.clone().untyped().type_id() != TypeId::of::<LoadedUntypedAsset>() {
                return Ok(Self(v.0));
            }
        };

        world
            .clone()
            .with_resource(|untyped_handles: &Assets<LoadedUntypedAsset>| {
                V::<Handle<LoadedUntypedAsset>>::from_script(value, world).and_then(move |h| {
                    let handle_to_untyped_handle = h.into_inner();

                    let untyped_handle = untyped_handles
                        .get(&handle_to_untyped_handle)
                        .ok_or_else(|| InteropError::str("Untyped asset was missing"))?;

                    let typed_handle = untyped_handle
                        .handle
                        .clone()
                        .try_typed()
                        .map_err(InteropError::external)?;

                    Ok(Self(typed_handle))
                })
            })?
    }
}

pub fn load_untyped_asset_for_script_descriptor(
    mod_name: &str,
    path: &str,
    script_descriptor_assets: &Assets<ModDescriptorAsset>,
    loaded_script_descriptors: &ModDescriptorLoadedAssets,
    asset_server: Mut<AssetServer>,
) -> Result<Option<ScriptHandleWrapper<LoadedUntypedAsset>>, ParseAssetPathError> {
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

            Ok(Some(
                asset_server.load_untyped(mod_relative_asset_path).into(),
            ))
        }
        _ => Ok(None),
    }
}
