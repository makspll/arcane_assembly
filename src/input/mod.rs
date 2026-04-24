use bevy::{
    app::{Plugin, PreUpdate, Update},
    input::{keyboard::KeyCode, mouse::MouseButton},
};
use bevy_mod_scripting::prelude::ScriptValue;

use crate::input::mouse::{MousePositionInWorldCoordinates, compute_mouse_world_position};
pub mod mouse;

#[derive(Clone, Copy)]
pub enum PlayerInput {
    Left,
    Right,
    Jump,
    Fire,
    Unknown,
}

impl From<&KeyCode> for PlayerInput {
    fn from(value: &KeyCode) -> Self {
        match value {
            KeyCode::KeyW | KeyCode::Space | KeyCode::Backspace => Self::Jump,
            KeyCode::KeyA => Self::Left,
            KeyCode::KeyD => Self::Right,
            _ => PlayerInput::Unknown,
        }
    }
}

impl From<&MouseButton> for PlayerInput {
    fn from(value: &MouseButton) -> Self {
        match value {
            MouseButton::Left => PlayerInput::Fire,
            _ => PlayerInput::Unknown,
        }
    }
}

impl From<PlayerInput> for ScriptValue {
    fn from(val: PlayerInput) -> Self {
        match val {
            PlayerInput::Left => ScriptValue::String("left".into()),
            PlayerInput::Right => ScriptValue::String("right".into()),
            PlayerInput::Jump => ScriptValue::String("jump".into()),
            PlayerInput::Fire => ScriptValue::String("fire".into()),
            _ => ScriptValue::String("unknown".into()),
        }
    }
}

pub struct GameInputPlugin;

impl Plugin for GameInputPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.register_type::<MousePositionInWorldCoordinates>()
            .init_resource::<MousePositionInWorldCoordinates>();

        app.add_systems(PreUpdate, compute_mouse_world_position);
    }
}
