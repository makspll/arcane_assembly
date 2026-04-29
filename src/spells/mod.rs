use bevy::{
    app::{Plugin, PostUpdate, Update},
    asset::AssetApp,
    ecs::schedule::IntoScheduleConfigs,
};
use bevy_mod_scripting::{lua::LuaScriptingPlugin, prelude::event_handler};

use crate::{
    spells::{
        bindings::register_global_spells_functions,
        executor::{
            AbilityExecution, AbilityExecutions, SpellEvent, progress_spell_executions,
            read_spell_events_into_executor, spell_executions_live,
        },
        lifecycle::{
            despawn_expired_entities, mark_dead_lifetimes, trigger_spell_expirations,
            trigger_spell_hits,
        },
        mana::Mana,
        spell_component_asset::SpellComponentAsset,
    },
    system_sets::GameSystemSets,
};

mod bindings;
pub mod dotgraph;
pub mod executor;
pub mod lifecycle;
pub mod mana;
pub mod spell;
pub mod spell_component_asset;

pub struct GameSpellsPlugin;

impl Plugin for GameSpellsPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_message::<SpellEvent>()
            .init_resource::<AbilityExecutions>()
            .init_asset::<SpellComponentAsset>();

        // rapier runs in update
        // our scripts will have to react accordingly
        app.add_systems(
            Update,
            (
                mark_dead_lifetimes.in_set(GameSystemSets::LifetimeDespawning),
                trigger_spell_hits.in_set(GameSystemSets::SpellDispatch),
                trigger_spell_expirations.in_set(GameSystemSets::SpellDispatch),
                (
                    read_spell_events_into_executor,
                    (progress_spell_executions.run_if(spell_executions_live)),
                )
                    .in_set(GameSystemSets::SpellHandling)
                    .chain(),
                despawn_expired_entities.in_set(GameSystemSets::LifetimeDespawning),
            ),
        );

        app.register_type::<Mana>();

        register_global_spells_functions(app.world_mut());
    }
}
