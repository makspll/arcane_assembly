use bevy::input::keyboard::KeyCode;
use bevy_mod_scripting::prelude::ScriptValue;

#[derive(Clone, Copy)]
pub enum PlayerInput {
    Left,
    Right,
    Jump,
    Unknown,
}

impl From<&KeyCode> for PlayerInput {
    fn from(value: &KeyCode) -> Self {
        match value {
            KeyCode::KeyW | KeyCode::Space => Self::Jump,
            KeyCode::KeyA => Self::Left,
            KeyCode::KeyD => Self::Right,
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
            _ => ScriptValue::String("unknown".into()),
        }
    }
}
