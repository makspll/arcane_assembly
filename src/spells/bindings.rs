use crate::{
    scripts::{
        loaded_script_descriptors::LoadedScriptDescriptors, script_descriptor::ScriptDescriptor,
    },
    spells::spell::LiveSpell,
};
use bevy::{
    asset::Assets,
    ecs::entity::Entity,
    math::Vec2,
    prelude::World,
    time::{Time, Virtual},
};
use bevy_mod_scripting::{
    bindings::{FunctionCallContext, InteropError, V},
    script_bindings,
};

#[script_bindings(name = "global_spells_functions", remote, unregistered)]
impl World {
    /// Instantiates a spell from its mod name and spell name
    /// Arguments:
    /// * `mod_name` - the name of the mod from which to spawn the spell
    /// * `spell_name` - the friendly name of the spell
    /// Returns:
    /// * `entity` - the entity bearing the spell.
    fn spawn_spell(
        ctxt: FunctionCallContext,
        mod_name: String,
        spell_name: String,
        position: V<Vec2>,
        velocity: V<Vec2>,
    ) -> Result<Option<V<Entity>>, InteropError> {
        let world = ctxt.world()?;

        // we need mut access to queue commands anyway, reduce checks
        world.with_world_mut_access(|w| {
            let descriptors = w
                .get_resource::<LoadedScriptDescriptors>()
                .expect("invariant");

            let assets = w
                .get_resource::<Assets<ScriptDescriptor>>()
                .expect("invariant");

            let spell_descriptor =
                descriptors.get_mod_spell_by_name(&mod_name, &spell_name, assets);

            let spell_descriptor = match spell_descriptor {
                Some(d) => d.clone(),
                None => return Ok(None),
            };

            let time = w.get_resource::<Time<Virtual>>().expect("invariant");

            let spell = LiveSpell::new(*position, *velocity, time, spell_descriptor);

            let entity = w.spawn(spell).id();
            // immediately apply command
            w.flush();
            Ok(Some(V::new(entity)))
        })?
    }
}
