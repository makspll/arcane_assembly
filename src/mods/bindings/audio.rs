use std::any::TypeId;

use crate::audio::{handle::AudioSourceHandle, start_sound_effect};
use bevy::{asset::LoadedUntypedAsset, audio::AudioSource, ecs::world::World};
use bevy_mod_scripting::{
    bindings::{FunctionCallContext, InteropError},
    script_bindings,
};

#[script_bindings(name = "audio_global_functions", remote, unregistered)]
impl World {
    /// Loads asset from the given mod, relative to the directory containing ".mod.json" files
    /// Arguments:
    /// * `handle` - the handle to the audio to be played.
    /// Returns:
    /// * `untyped_handle` - the handle to this asset, which when the asset is fully loaded will be accessible
    fn play_sound_effect(
        ctxt: FunctionCallContext,
        handle: AudioSourceHandle,
    ) -> Result<(), InteropError> {
        let world = ctxt.world()?;

        world.with_world_mut_access(|w| {
            start_sound_effect(w.commands(), handle.0);
        })?;
        Ok(())
    }
}
