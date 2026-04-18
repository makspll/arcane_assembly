//! Wrappers for bevy_aseprite types which support reflection

use crate::physics::PIXELS_PER_METER;
use bevy::{
    asset::{Assets, Handle, LoadedUntypedAsset},
    ecs::{bundle::Bundle, entity::Entity, system::Commands},
    math::Vec2,
    reflect::{Reflect, TypeRegistry, Typed},
    sprite::Sprite,
};
use bevy_aseprite_ultra::prelude::{Animation, AseAnimation, Aseprite};
use bevy_mod_scripting::{
    GetTypeDependencies,
    bindings::{ArgMeta, FromScript, InteropError, TypedThrough, V, WorldExtensions},
    display::WorldAccessGuard,
    prelude::ScriptValue,
};

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
        value: ScriptValue,
        world: WorldAccessGuard<'_>,
    ) -> Result<Self::This<'_>, bevy_mod_scripting::bindings::InteropError>
    where
        Self: Sized,
    {
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
            // camera is scaled such that we align with physics units
            // size is in meters here, 1m = 32px
            custom_size: Some(Vec2 { x: 1.0, y: 1.0 }),
            ..Default::default()
        },
    });
}
