use crate::{
    mods::{
        mod_descriptor_asset::ModDescriptorAsset,
        mod_descriptor_loaded_assets::{self, ModDescriptorLoadedAssets},
    },
    spells::{
        dotgraph::dot_graph_to_spell_graph,
        executor::{CastSpell, Spell},
        spell::LiveSpell,
    },
};
use bevy::{
    asset::Assets,
    ecs::entity::Entity,
    math::Vec2,
    prelude::World,
    time::{Time, Virtual},
};
use bevy_mod_scripting::{
    bindings::{FunctionCallContext, InteropError, R, V, WorldExtensions},
    script_bindings,
};

#[script_bindings(name = "global_spells_functions", remote, unregistered)]
impl World {
    /// Creates a spell from a dotgraph string. The dotgraph must contain at least one node with ID (not label) of 'Start'.
    /// The graph nodes must be labelled via "label=""" attributes in the format: "<mod_name,>spell_name", the mod name is optional.
    /// The graph edges may be labelled with the callback prefix.
    /// The execution of the spell won't progress to the next spell component if there is at least one edge from the current component to another with
    /// a callback marker that hasn't been triggered each frame.
    /// Arguments:
    /// * `as_mod` - the mod to use if no mod name is provided on a node
    /// * `graph` - the dotgraph string
    fn parse_spell(
        ctxt: FunctionCallContext,
        as_mod: String,
        graph: String,
    ) -> Result<V<Spell>, InteropError> {
        let world = ctxt.world()?;
        world.with_resource(|descriptors: &ModDescriptorLoadedAssets| {
            world.with_resource(|assets: &Assets<ModDescriptorAsset>| {
                dot_graph_to_spell_graph(&graph, &as_mod, descriptors, assets)
                    .map_err(InteropError::string)
                    .map(V::new)
            })
        })??
    }

    /// Instantiates a spell at the given location with the given velocity
    /// Arguments:
    /// * `spell` - the spell to cast
    /// * `position` - the position of the cast
    /// * `velocity` - the velocity of the cast
    fn cast_spell(
        ctxt: FunctionCallContext,
        spell: V<Spell>,
        caster: V<Entity>,
        position: V<Vec2>,
        velocity: V<Vec2>,
    ) -> Result<(), InteropError> {
        let world = ctxt.world()?;

        // we need mut access to queue commands anyway, reduce checks
        world.with_world_mut_access(|w| {
            w.commands()
                // could do ref but we need to clone anyway
                .queue(CastSpell::new(
                    *caster,
                    spell.into_inner(),
                    *position,
                    *velocity,
                ));

            // immediately apply command
            w.flush();
            Ok(())
        })?
    }
}
