use bevy::{prelude::*, window::WindowResolution};
use bevy_aseprite_ultra::AsepriteUltraPlugin;
use bevy_mod_scripting::{ladfile::plugin::ScriptingFilesGenerationPlugin, prelude::*};
use std::path::*;

use crate::{
    audio::GameAudioPlugin,
    character::CharacterPlugin,
    console::DevConsolePlugin,
    map::MapPlugin,
    physics::PhysicsPlugin,
    scripts::{ScriptLoaderPlugin, bindings::ScriptBindingsPlugin},
    settings::WindowSettings,
    sprite::SpritesPlugin,
    state::ArcaneAssemblyGameStatePlugin,
};

mod audio;
mod camera;
mod character;
mod console;
mod input;
mod map;
mod physics;
mod scripts;
mod settings;
mod sprite;
mod state;

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

        #[cfg(debug_assertions)] // debug/dev builds only
        {
            use bevy::diagnostic::LogDiagnosticsPlugin;
            use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
            app.add_plugins(LogDiagnosticsPlugin::default())
                .add_plugins(EguiPlugin::default())
                .add_plugins(WorldInspectorPlugin::new());
        }

        // Bevy Mod Scripting Framework
        app.add_plugins(BMSPlugin.set::<ScriptingFilesGenerationPlugin>(
            ScriptingFilesGenerationPlugin::new(
                true, // enabled, you can use a compilation feature to disable this here
                PathBuf::from("assets").join("definitions"),
                Some(PathBuf::from("bindings.lad.json")), // do also save the ladfile itself
                "Arcane Assembly LUA Interface",
                true,
                true,
            ),
        ));

        // Dependencies
        app.add_plugins(AsepriteUltraPlugin);
        // app.register_type::<Aseprite>();

        // Game Plugins
        app.add_plugins((
            ScriptLoaderPlugin,
            DevConsolePlugin,
            crate::camera::CameraPlugin,
            ArcaneAssemblyGameStatePlugin,
            CharacterPlugin,
            PhysicsPlugin,
            MapPlugin,
            ScriptBindingsPlugin,
            SpritesPlugin,
            GameAudioPlugin,
        ));
    }
}
