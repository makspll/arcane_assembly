use bevy::{prelude::*, window::WindowResolution};
use bevy_mod_scripting::{ladfile::plugin::ScriptingFilesGenerationPlugin, prelude::*};
use std::path::*;

use crate::{
    console::DevConsolePlugin, scripts::ScriptLoaderPlugin, settings::WindowSettings,
    state::ArcaneAssemblyGameStatePlugin,
};

mod camera;
mod console;
mod scripts;
mod settings;
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
            DefaultPlugins.set(WindowPlugin {
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
            }),
        );

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

        // Game Plugins
        app.add_plugins((
            ScriptLoaderPlugin,
            DevConsolePlugin,
            crate::camera::CameraPlugin,
            ArcaneAssemblyGameStatePlugin,
        ));
    }
}
