use bevy::audio::AudioSource;

use crate::scripts::assets::ScriptHandleWrapper;

/// A newtype around [`Handle<AudioSource>`], with de-sugaring implemented for script binding code.
pub type AudioSourceHandle = ScriptHandleWrapper<AudioSource>;
