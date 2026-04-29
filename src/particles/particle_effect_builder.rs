use std::sync::Arc;

use bevy::{asset::Handle, reflect::Reflect};
use bevy_hanabi::{
    AlphaMode, Attribute, EffectAsset, ExprHandle, Modifier, ModifierContext, Module,
    MotionIntegration, RenderModifier, SimulationCondition, SimulationSpace, SpawnerSettings,
    Value,
};

#[derive(Clone, Reflect, Default)]
#[reflect(opaque)]
pub struct ParticleEffectModule {
    pub module: Module,
}

#[derive(Clone, Reflect)]
#[reflect(opaque)]
pub struct ParticleExprHandle {
    pub expr: ExprHandle,
}

#[derive(Clone, Reflect)]
#[reflect(opaque)]
pub struct ParticleAttribute {
    pub attr: Attribute,
}

#[derive(Clone, Reflect, Default)]
#[reflect(opaque)]
pub struct ParticleEffectBuilder {
    pub capacity: Option<u32>,
    pub spawn_particle_count: Option<f32>,
    pub spawn_cycle_duration: Option<f32>,
    pub spawn_cycle_period: Option<f32>,
    pub spawn_cycle_count: Option<u32>,
    pub init_modifiers: Vec<Arc<dyn Modifier + Send + Sync>>,
    pub update_modifiers: Vec<Arc<dyn Modifier + Send + Sync>>,
    pub render_modifiers: Vec<Arc<dyn RenderModifier + Send + Sync>>,
    pub name: Option<String>,
    pub simulation_space: Option<SimulationSpace>,
    pub simulation_condition: Option<SimulationCondition>,
    pub alpha_mode: Option<AlphaMode>,
    pub motion_integration: Option<MotionIntegration>,
    pub mesh: Option<Handle<bevy::mesh::Mesh>>,
    pub module: Option<ParticleEffectModule>,
}

impl ParticleEffectBuilder {
    pub fn build(self) -> EffectAsset {
        let mut effect = EffectAsset::new(
            self.capacity.unwrap_or(1000),
            SpawnerSettings::new(
                self.spawn_particle_count.unwrap_or(100.0).into(),
                self.spawn_cycle_duration.unwrap_or(0.0).into(),
                self.spawn_cycle_period.unwrap_or(1.0).into(),
                self.spawn_cycle_count.unwrap_or(1),
            ),
            self.module.unwrap_or_default().module,
        );

        if let Some(name) = self.name {
            effect = effect.with_name(name);
        }

        if let Some(simulation_space) = self.simulation_space {
            effect = effect.with_simulation_space(simulation_space);
        }
        if let Some(simulation_condition) = self.simulation_condition {
            effect = effect.with_simulation_condition(simulation_condition);
        }
        if let Some(alpha_mode) = self.alpha_mode {
            effect = effect.with_alpha_mode(alpha_mode);
        }
        if let Some(motion_integration) = self.motion_integration {
            effect = effect.with_motion_integration(motion_integration);
        }
        if let Some(mesh) = self.mesh {
            effect = effect.mesh(mesh)
        }

        for m in self.init_modifiers {
            effect = effect.add_modifier(ModifierContext::Init, m.boxed_clone());
        }
        for m in self.update_modifiers {
            effect = effect.add_modifier(ModifierContext::Update, m.boxed_clone());
        }
        for m in self.render_modifiers {
            effect = effect.add_render_modifier(m.boxed_render_clone());
        }

        effect
    }
}

#[derive(Clone, Reflect)]
#[reflect(opaque)]
pub struct ParticleModifier {
    pub inner: Arc<dyn Modifier + Send + Sync>,
}

#[derive(Clone, Reflect)]
#[reflect(opaque)]
pub struct ParticleRenderModifier {
    pub inner: Arc<dyn RenderModifier + Send + Sync>,
}
