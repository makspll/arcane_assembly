use bevy::{
    app::Plugin,
    asset::Handle,
    audio::{AudioPlayer, AudioSource, PlaybackSettings},
    ecs::system::Commands,
};

pub mod handle;

/// Spawns an entity which will create all the necessary components for audio playback, then despawn itself
/// Multiple sound effects can play in parallel.
///
/// At the moment everything plays in the same channel and volume
pub fn start_sound_effect(mut commands: Commands, source: Handle<AudioSource>) {
    commands.spawn((
        AudioPlayer::new(source),
        PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Despawn,
            volume: bevy::audio::Volume::Linear(0.1), //because I value my eardrums
            ..Default::default()
        },
    ));
}

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut bevy::app::App) {}
}
