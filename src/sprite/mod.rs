use bevy::app::Plugin;

pub mod aseprite;

pub struct SpritesPlugin;

impl Plugin for SpritesPlugin {
    fn build(&self, app: &mut bevy::app::App) {}
}
