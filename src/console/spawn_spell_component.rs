use bevy::{
    asset::Assets,
    ecs::system::{Commands, Res},
    math::Vec2,
    time::{Time, Virtual},
};
use bevy_console::{ConsoleCommand, clap};

use crate::{
    scripts::{
        loaded_script_descriptors::LoadedScriptDescriptors, script_descriptor::ScriptDescriptor,
    },
    spells::spell::LiveSpell,
};

#[derive(clap::Parser, ConsoleCommand)]
#[command(name = "spawn_spell")]
pub struct SpawnSpellComponentCmnd {
    pub mod_name: String,
    pub spell_name: String,
    #[clap(default_value = "0")]
    pub pos_x: f32,
    #[clap(default_value = "0")]
    pub pos_y: f32,
    #[clap(default_value = "10")]
    pub vel_x: f32,
    #[clap(default_value = "0.1")]
    pub vel_y: f32,
}

pub fn spawn_spell_component_cmnd(
    mut log: ConsoleCommand<SpawnSpellComponentCmnd>,
    mut commands: Commands,
    descriptors: Res<LoadedScriptDescriptors>,
    descriptor_assets: Res<Assets<ScriptDescriptor>>,
    time: Res<Time<Virtual>>,
) {
    if let Some(Ok(cmd)) = log.take() {
        let mod_descriptor = match descriptors.get_mod_by_name(&cmd.mod_name, &descriptor_assets) {
            Some((mod_descriptor, _)) => mod_descriptor,
            None => {
                log.reply_failed(format!("Mod not found: {}", cmd.mod_name));
                return;
            }
        };
        let spell_component = match mod_descriptor
            .spell_components
            .iter()
            .find(|s| s.friendly_name == cmd.spell_name)
        {
            Some(s) => s,
            None => {
                log.reply_failed(format!("Spell not found in mod: {}", cmd.spell_name));
                return;
            }
        };

        commands.spawn(LiveSpell::new(
            Vec2::new(cmd.pos_x, cmd.pos_y),
            Vec2::new(cmd.vel_x, cmd.vel_y),
            time,
            spell_component.clone(),
        ));
    }
}
