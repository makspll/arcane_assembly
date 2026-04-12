//! Wrappers for bevy_aseprite types which support reflection

use std::{any::TypeId, error::Error, sync::Arc};

use bevy::{
    asset::{
        Asset, AssetLoader, Assets, Handle, LoadedUntypedAsset, transformer::AssetTransformer,
    },
    ecs::{
        bundle::Bundle,
        entity::Entity,
        error::BevyError,
        system::{Commands, In},
        world::Mut,
    },
    math::Vec2,
    reflect::{DynamicTyped, FromReflect, Reflect, TypePath, TypeRegistry, Typed},
    sprite::Sprite,
};
use bevy_aseprite_ultra::prelude::{Animation, AseAnimation, Aseprite};
use bevy_mod_scripting::{
    GetTypeDependencies, TypedThrough,
    bindings::{
        ArgMeta, FromScript, InteropError, ReflectBase, ReflectReference, ScriptArgument,
        TypedThrough, Val,
    },
    ladfile::ReflectionPrimitiveKind,
    prelude::ScriptValue,
};

use crate::physics::{METERS_PER_PIXEL, PIXELS_PER_METER};

/// A newtype around [`Handle<Aseprite>`], with de-sugaring implemented for script binding code.
#[derive(Clone, Debug, Reflect, GetTypeDependencies)]
#[reflect(opaque)]
pub struct AsepriteHandle(pub Handle<Aseprite>);

impl TypedThrough for AsepriteHandle {
    fn through_type_info() -> bevy_mod_scripting::bindings::ThroughTypeInfo {
        bevy_mod_scripting::bindings::ThroughTypeInfo::TypeInfo(AsepriteHandle::type_info())
    }
}

impl ArgMeta for AsepriteHandle {}

impl FromScript for AsepriteHandle {
    type This<'w> = Self;

    fn from_script(
        value: bevy_mod_scripting::prelude::ScriptValue,
        world: bevy_mod_scripting::bindings::WorldGuard<'_>,
    ) -> Result<Self::This<'_>, bevy_mod_scripting::bindings::InteropError>
    where
        Self: Sized,
    {
        // if let ScriptValue::Reference(ref ref_) = value
        //     && let ReflectBase::Asset(untyped_handle, _) = &ref_.base.base_id
        //     && let Ok(aseprite_handle) = untyped_handle.clone().try_typed::<Aseprite>()
        // {
        //     return Ok(Self(aseprite_handle));
        // }
        world
            .clone()
            .with_resource(|untyped_handles: &Assets<LoadedUntypedAsset>| {
                Val::<Handle<LoadedUntypedAsset>>::from_script(value, world).and_then(move |h| {
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

#[derive(Bundle)]
struct AsepriteBundle {
    spritesheet: AseAnimation,
    sprite: Sprite,
}

pub fn set_aseprite_animation_on_entity(
    mut cmd: Commands,
    spritesheet: Handle<Aseprite>,
    entity: Entity,
    animation_tag: &str,
) {
    cmd.entity(entity).insert_if_new(AsepriteBundle {
        spritesheet: AseAnimation {
            animation: Animation::tag(animation_tag),
            aseprite: spritesheet,
        },
        sprite: Sprite {
            custom_size: Some(Vec2 {
                x: PIXELS_PER_METER,
                y: PIXELS_PER_METER,
            }),
            ..Default::default()
        },
    });
}
