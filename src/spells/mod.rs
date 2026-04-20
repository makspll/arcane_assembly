use bevy::{
    app::{Plugin, Update},
    ecs::schedule::IntoScheduleConfigs,
};
use bevy_mod_scripting::{lua::LuaScriptingPlugin, prelude::event_handler};

use crate::spells::{callbacks::OnSpellExpired, spell::expire_dead_spells};

pub mod callbacks;
pub mod spell;

pub struct GameSpellsPlugin;

impl Plugin for GameSpellsPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        // does this need to be ordered relatively to other event handlers like on update ?
        app.add_systems(
            Update,
            expire_dead_spells.before(event_handler::<OnSpellExpired, LuaScriptingPlugin>),
        );
        app.add_systems(Update, event_handler::<OnSpellExpired, LuaScriptingPlugin>);
    }
}
