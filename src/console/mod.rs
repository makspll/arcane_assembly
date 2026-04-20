use bevy::app::Plugin;
use bevy_console::{AddConsoleCommand, ConsolePlugin};

use crate::console::{
    list_mods::{ListModsCmnd, list_mods_cmd},
    spawn_spell_component::{SpawnSpellComponentCmnd, spawn_spell_component_cmnd},
};

mod list_mods;
mod spawn_spell_component;

pub struct DevConsolePlugin;

impl Plugin for DevConsolePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(ConsolePlugin);
        app.add_console_command::<ListModsCmnd, _>(list_mods_cmd);
        app.add_console_command::<SpawnSpellComponentCmnd, _>(spawn_spell_component_cmnd);
    }
}
