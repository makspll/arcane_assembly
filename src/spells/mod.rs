use bevy::{
    app::{Plugin, PostUpdate, Update},
    ecs::schedule::IntoScheduleConfigs,
};
use bevy_mod_scripting::{lua::LuaScriptingPlugin, prelude::event_handler};

use crate::spells::{
    bindings::register_global_spells_functions,
    callbacks::{OnSpellCast, OnSpellExpired, OnSpellHitCharacter, OnSpellHitTerrain},
    mana::Mana,
    spell::{trigger_spell_expirations, trigger_spell_hits},
};

mod bindings;
pub mod callbacks;
pub mod mana;
pub mod spell;
pub struct GameSpellsPlugin;

impl Plugin for GameSpellsPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        // TODO: does this need to be ordered relatively to other event handlers like on update ?
        // eventually refactor into system sets across the whole app
        app.add_systems(
            Update,
            (
                event_handler::<OnSpellCast, LuaScriptingPlugin>,
                trigger_spell_expirations,
                event_handler::<OnSpellExpired, LuaScriptingPlugin>,
            )
                .chain(),
        );
        // rapier runs in update
        // our scripts will have to react accordingly
        app.add_systems(
            PostUpdate,
            (
                trigger_spell_hits,
                event_handler::<OnSpellHitCharacter, LuaScriptingPlugin>,
                event_handler::<OnSpellHitTerrain, LuaScriptingPlugin>,
            )
                .chain(),
        );

        app.register_type::<Mana>();

        register_global_spells_functions(app.world_mut());
    }
}
