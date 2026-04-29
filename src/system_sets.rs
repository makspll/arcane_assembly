use bevy::prelude::IntoScheduleConfigs;
use bevy::{app::Plugin, ecs::schedule::SystemSet};
#[derive(SystemSet, Hash, Debug, PartialEq, Eq, Clone)]
pub enum GameSystemSets {
    /// systems which dispatch on update events
    UpdateDispatch,
    /// systems which dispatch player input related events
    PlayerInputDispatch,

    /// systems which handle on update events
    UpdateHandling,
    /// systems which handle player input related events
    PlayerInputHandling,
    /// Systems which mark WithLifetime components as expired
    LifetimeExpiry,
    /// Systems which trigger spell events
    SpellDispatch,
    /// systems which handle spell events
    SpellHandling,
    /// Systems which despawn entities marked as expired in LifetimeExpiry
    LifetimeDespawning,
}

pub struct GameSystemSetPlugin;

impl Plugin for GameSystemSetPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        use GameSystemSets::*;
        app.configure_sets(
            bevy::prelude::Update,
            (
                UpdateDispatch,
                PlayerInputDispatch,
                LifetimeDespawning,
                SpellDispatch,
                SpellHandling,
                UpdateHandling,
                PlayerInputHandling,
                LifetimeExpiry,
            )
                .chain(),
        );
    }
}
