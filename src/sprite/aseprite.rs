//! Wrappers for bevy_aseprite types which support reflection

use crate::scripts::assets::ScriptHandleWrapper;
use bevy::{
    asset::Handle,
    ecs::{bundle::Bundle, entity::Entity, system::Commands},
    math::Vec2,
    sprite::Sprite,
};
use bevy_aseprite_ultra::prelude::{Animation, AseAnimation, Aseprite};

/// A newtype around [`Handle<Aseprite>`], with de-sugaring implemented for script binding code.
pub type AsepriteHandle = ScriptHandleWrapper<Aseprite>;

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
    flip_sprite: bool,
) {
    let animation_tag = animation_tag.to_owned();
    let mut entity_cmds = cmd.entity(entity);
    let entity_cmds = entity_cmds.insert_if_new(AsepriteBundle {
        spritesheet: AseAnimation {
            animation: Animation::tag(&animation_tag),
            aseprite: spritesheet,
        },
        sprite: Sprite {
            // camera is scaled such that we align with physics units
            // size is in meters here, 1m = 32px
            custom_size: Some(Vec2 { x: 1.0, y: 1.0 }),
            ..Default::default()
        },
    });

    entity_cmds
        .entry::<AseAnimation>()
        .and_modify(move |mut a| a.animation = Animation::tag(&animation_tag));
    entity_cmds
        .entry::<Sprite>()
        .and_modify(move |mut s| s.flip_x = flip_sprite);
}
