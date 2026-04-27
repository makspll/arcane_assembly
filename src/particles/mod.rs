use bevy::app::Plugin;
use bevy_hanabi::{HanabiPlugin, ParticleEffect};

use crate::particles::{
    bindings::{
        register_alpha_mode_bindings, register_color_blend_mask_bindings,
        register_color_blend_mode_bindings, register_global_particle_functions,
        register_motion_integration_bindings, register_particle_attribute_bindings,
        register_particle_effect_builder, register_particle_modifier_bindings,
        register_particle_module_bindings, register_particle_render_modifier_bindings,
        register_shape_dimension_bindings, register_simulation_condition_bindings,
        register_simulation_space_bindings,
    },
    particle_effect_builder::{
        ParticleAttribute, ParticleEffectBuilder, ParticleEffectModule, ParticleExprHandle,
    },
};
mod bindings;

pub fn clean_up_dead_particles(query: Query<ParticleEffect>) {}

pub mod particle_effect_builder;
pub struct GameParticlesPlugin;

impl Plugin for GameParticlesPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(HanabiPlugin)
            .register_type::<ParticleEffectBuilder>()
            .register_type::<ParticleAttribute>()
            .register_type::<ParticleExprHandle>()
            .register_type::<ParticleEffectModule>();
        register_particle_effect_builder(app.world_mut());
        register_global_particle_functions(app.world_mut());
        register_particle_attribute_bindings(app.world_mut());
        register_particle_attribute_bindings(app.world_mut());
        register_particle_module_bindings(app.world_mut());
        register_particle_modifier_bindings(app.world_mut());
        register_particle_render_modifier_bindings(app.world_mut());
        register_motion_integration_bindings(app.world_mut());
        register_simulation_condition_bindings(app.world_mut());
        register_simulation_space_bindings(app.world_mut());
        register_alpha_mode_bindings(app.world_mut());
        register_shape_dimension_bindings(app.world_mut());
        register_color_blend_mask_bindings(app.world_mut());
        register_color_blend_mode_bindings(app.world_mut());
    }
}
