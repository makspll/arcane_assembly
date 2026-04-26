use bevy::{prelude::*, window::WindowResolution};
use bevy_aseprite_ultra::AsepriteUltraPlugin;
use bevy_mod_scripting::{ladfile::plugin::ScriptingFilesGenerationPlugin, prelude::*};
use std::path::*;

use crate::{
    audio::GameAudioPlugin,
    character::CharacterPlugin,
    console::DevConsolePlugin,
    input::GameInputPlugin,
    map::MapPlugin,
    mods::{ScriptLoaderPlugin, bindings::ScriptBindingsPlugin},
    physics::PhysicsPlugin,
    settings::WindowSettings,
    spells::GameSpellsPlugin,
    sprite::SpritesPlugin,
    state::ArcaneAssemblyGameStatePlugin,
    system_sets::GameSystemSetPlugin,
    ui::GameUiPlugin,
};

mod audio;
mod camera;
mod character;
mod console;
mod input;
mod map;
mod mods;
mod physics;
mod settings;
mod spells;
mod sprite;
mod state;
mod system_sets;
mod ui;

fn main() -> AppExit {
    App::new().add_plugins(ArcaneAssemblyPlugin).run()
}

pub struct ArcaneAssemblyPlugin;

impl Plugin for ArcaneAssemblyPlugin {
    fn build(&self, app: &mut App) {
        // todo: drive from UI and smart defaults
        let window_settings = WindowSettings::default();

        // Add Bevy plugins.
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "Arcane Assembly".to_string(),
                        fit_canvas_to_parent: true,
                        resolution: WindowResolution::new(
                            window_settings.width,
                            window_settings.height,
                        ),
                        ..default()
                    }
                    .into(),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        );

        // dev builds only, add anything we don't wanna ship into prod
        #[cfg(feature = "dev_tools")]
        {
            info!("running dev build");
            use bevy_dev_tools::fps_overlay::FpsOverlayPlugin;
            use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

            use crate::mods::systems::sync_dev_schema;
            app.add_plugins(FpsOverlayPlugin::default())
                .add_plugins(EguiPlugin::default())
                .add_plugins(WorldInspectorPlugin::new());
            // we can distribute this however, but it's nice to keep in sync automatically
            app.add_systems(Startup, sync_dev_schema);
        }

        // Dependencies
        app.add_plugins(AsepriteUltraPlugin);
        // app.register_type::<Aseprite>();

        // Game Plugins
        app.add_plugins((
            DevConsolePlugin,
            ScriptLoaderPlugin,
            crate::camera::CameraPlugin,
            ArcaneAssemblyGameStatePlugin,
            CharacterPlugin,
            PhysicsPlugin,
            MapPlugin,
            ScriptBindingsPlugin,
            SpritesPlugin,
            GameAudioPlugin,
            GameSpellsPlugin,
            GameInputPlugin,
            GameUiPlugin,
            GameSystemSetPlugin,
        ));
    }
}
