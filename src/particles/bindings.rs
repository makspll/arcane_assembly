use std::{f32, sync::Arc};

use bevy::{
    asset::{Assets, Handle},
    color::LinearRgba,
    ecs::{
        entity::Entity,
        world::{Mut, World},
    },
    math::{Vec2, Vec3, Vec4},
    mesh::Mesh,
    reflect::Reflect,
    transform::components::Transform,
};
use bevy_hanabi::prelude::*;
use bevy_mod_scripting::{bindings::*, script_bindings};

use crate::{
    mods::assets::ScriptHandleWrapper,
    particles::particle_effect_builder::{
        ParticleAttribute, ParticleEffectBuilder, ParticleEffectModule, ParticleExprHandle,
        ParticleModifier, ParticleRenderModifier,
    },
};

// -- Globals --
#[script_bindings(name = "global_particle_functions", remote, unregistered)]
impl World {
    /// Spawns a one-shot particle effect at a position
    fn spawn_particle_effect(
        ctxt: FunctionCallContext,
        effect: V<ScriptHandleWrapper<EffectAsset>>,
        position: V<Vec3>,
    ) -> Result<V<Entity>, InteropError> {
        let world = ctxt.world()?;

        world.with_world_mut_access(|w| {
            let entity = w
                .commands()
                .spawn((
                    ParticleEffect::new(effect.0.0),
                    Transform::from_translation(*position),
                ))
                .id();

            w.flush();

            Ok(V::new(entity))
        })?
    }

    // -- Enum Constructors --
}

#[script_bindings(name = "color_blend_mode_bindings", remote)]

impl ColorBlendMode {
    fn overwrite() -> V<ColorBlendMode> {
        ColorBlendMode::Overwrite.into()
    }
    fn add() -> V<ColorBlendMode> {
        ColorBlendMode::Add.into()
    }
    fn remove() -> V<ColorBlendMode> {
        ColorBlendMode::Modulate.into()
    }
}

#[script_bindings(name = "color_blend_mask_bindings", remote)]
impl ColorBlendMask {
    fn red() -> V<ColorBlendMask> {
        ColorBlendMask::R.into()
    }
    fn green() -> V<ColorBlendMask> {
        ColorBlendMask::G.into()
    }
    fn blue() -> V<ColorBlendMask> {
        ColorBlendMask::B.into()
    }
    fn rgb() -> V<ColorBlendMask> {
        ColorBlendMask::RGB.into()
    }
    fn rgba() -> V<ColorBlendMask> {
        ColorBlendMask::RGBA.into()
    }
}

#[script_bindings(name = "shape_dimension_bindings", remote)]
impl ShapeDimension {
    fn surface() -> V<ShapeDimension> {
        ShapeDimension::Surface.into()
    }
    fn volume() -> V<ShapeDimension> {
        ShapeDimension::Volume.into()
    }
}

#[script_bindings(name = "simulation_condition_bindings", remote)]
impl SimulationCondition {
    fn always() -> V<SimulationCondition> {
        SimulationCondition::Always.into()
    }
    fn when_visible() -> V<SimulationCondition> {
        SimulationCondition::WhenVisible.into()
    }
}

#[script_bindings(name = "simulation_space_bindings", remote)]
impl SimulationSpace {
    fn local() -> V<SimulationSpace> {
        SimulationSpace::Local.into()
    }

    fn global() -> V<SimulationSpace> {
        SimulationSpace::Global.into()
    }
}

#[script_bindings(name = "alpha_mode_bindings", remote)]
impl AlphaMode {
    fn blend() -> V<AlphaMode> {
        AlphaMode::Blend.into()
    }

    fn add() -> V<AlphaMode> {
        AlphaMode::Add.into()
    }

    fn pre_multiply() -> V<AlphaMode> {
        AlphaMode::Premultiply.into()
    }

    fn mask(mask: V<ParticleExprHandle>) -> V<AlphaMode> {
        AlphaMode::Mask(mask.expr).into()
    }
}

#[script_bindings(name = "motion_integration_bindings", remote)]
impl MotionIntegration {
    fn pre_update() -> V<MotionIntegration> {
        MotionIntegration::PreUpdate.into()
    }

    fn none() -> V<MotionIntegration> {
        MotionIntegration::None.into()
    }

    fn post_update() -> V<MotionIntegration> {
        MotionIntegration::PostUpdate.into()
    }
}

#[script_bindings(name = "particle_value_bindings", remote)]
impl Value {
    fn float(v: f32) -> V<Value> {
        Value::Scalar(ScalarValue::Float(v)).into()
    }

    fn bool(v: bool) -> V<Value> {
        Value::Scalar(ScalarValue::Bool(v)).into()
    }

    fn int(v: i32) -> V<Value> {
        Value::Scalar(ScalarValue::Int(v)).into()
    }

    fn uint(v: u32) -> V<Value> {
        Value::Scalar(ScalarValue::Uint(v)).into()
    }

    fn vec2(v: V<Vec2>) -> V<Value> {
        Value::Vector(VectorValue::new_vec2(*v)).into()
    }

    fn vec3(v: V<Vec3>) -> V<Value> {
        Value::Vector(VectorValue::new_vec3(*v)).into()
    }

    fn vec4(v: V<Vec4>) -> V<Value> {
        Value::Vector(VectorValue::new_vec4(*v)).into()
    }

    fn matrix(cols: usize, rows: usize, data: Vec<f32>) -> V<Value> {
        Value::Matrix(MatrixValue::new(cols, rows, data.as_slice())).into()
    }
}

// -- Modifier API --
#[script_bindings(name = "particle_modifier_bindings", remote)]
impl ParticleModifier {
    fn set_attribute(
        attribute: V<ParticleAttribute>,
        value: V<ParticleExprHandle>,
    ) -> V<ParticleModifier> {
        V::new(ParticleModifier {
            inner: Arc::new(SetAttributeModifier::new(attribute.attr, value.0.expr)),
        })
    }

    fn set_age(value: V<ParticleExprHandle>) -> V<ParticleModifier> {
        V::new(ParticleModifier {
            inner: Arc::new(SetAttributeModifier::new(Attribute::AGE, value.0.expr)),
        })
    }

    fn set_lifetime(value: V<ParticleExprHandle>) -> V<ParticleModifier> {
        V::new(ParticleModifier {
            inner: Arc::new(SetAttributeModifier::new(Attribute::LIFETIME, value.0.expr)),
        })
    }

    fn set_velocity(value: V<ParticleExprHandle>) -> V<ParticleModifier> {
        V::new(ParticleModifier {
            inner: Arc::new(SetAttributeModifier::new(Attribute::VELOCITY, value.0.expr)),
        })
    }

    fn set_position(value: V<ParticleExprHandle>) -> V<ParticleModifier> {
        V::new(ParticleModifier {
            inner: Arc::new(SetAttributeModifier::new(Attribute::POSITION, value.0.expr)),
        })
    }

    fn set_size(value: V<ParticleExprHandle>) -> V<ParticleModifier> {
        V::new(ParticleModifier {
            inner: Arc::new(SetAttributeModifier::new(Attribute::SIZE, value.0.expr)),
        })
    }

    fn position_circle(
        center: V<ParticleExprHandle>,
        radius: V<ParticleExprHandle>,
        dimension: V<ShapeDimension>,
        axis: V<ParticleExprHandle>,
    ) -> V<ParticleModifier> {
        V::new(ParticleModifier {
            inner: Arc::new(SetPositionCircleModifier {
                center: center.0.expr,
                radius: radius.0.expr,
                axis: axis.0.expr,
                dimension: *dimension,
            }),
        })
    }

    fn velocity_circle(
        center: V<ParticleExprHandle>,
        speed: V<ParticleExprHandle>,
        axis: V<ParticleExprHandle>,
    ) -> V<ParticleModifier> {
        V::new(ParticleModifier {
            inner: Arc::new(SetVelocityCircleModifier {
                center: center.0.expr,
                axis: axis.0.expr,
                speed: speed.0.expr,
            }),
        })
    }

    fn accel(accel: V<ParticleExprHandle>) -> V<ParticleModifier> {
        V::new(ParticleModifier {
            inner: Arc::new(AccelModifier::new(accel.0.expr)),
        })
    }

    fn linear_drag(drag: V<ParticleExprHandle>) -> V<ParticleModifier> {
        V::new(ParticleModifier {
            inner: Arc::new(LinearDragModifier { drag: drag.0.expr }),
        })
    }
}

#[script_bindings(name = "particle_render_modifier_bindings", remote, unregistered)]
impl ParticleRenderModifier {
    fn round(roundness: V<ParticleExprHandle>) -> V<ParticleRenderModifier> {
        ParticleRenderModifier {
            inner: Arc::new(RoundModifier {
                roundness: roundness.into_inner().expr,
            }),
        }
        .into()
    }

    fn color(
        color: V<Vec4>,
        blend_mode: V<ColorBlendMode>,
        mask: V<ColorBlendMask>,
    ) -> V<ParticleRenderModifier> {
        V::new(ParticleRenderModifier {
            inner: Arc::new(SetColorModifier {
                color: color.into_inner().into(),
                blend: blend_mode.into_inner(),
                mask: mask.into_inner(),
            }),
        })
    }

    // fn billboard() -> V<ParticleRenderModifier> {
    //     V::new(ParticleRenderModifier {
    //         inner: Arc::new(BillboardModifier),
    //     })
    // }

    // fn particle_texture(texture: Handle<Image>) -> V<ParticleRenderModifier> {
    //     V::new(ParticleRenderModifier {
    //         inner: Arc::new(ParticleTextureModifier {
    //             texture,
    //             texture_slot: todo!(),
    //             sample_mapping: todo!(),
    //         }),
    //     })
    // }

    // fn flipbook(columns: u32, rows: u32) -> V<ParticleRenderModifier> {
    //     V::new(ParticleRenderModifier {
    //         inner: Arc::new(FlipbookModifier {
    //             columns,
    //             rows,
    //             frame: 0,
    //         }),
    //     })
    // }
}

// -- Module Expression API --

#[script_bindings(name = "particle_attribute_bindings", remote)]
impl ParticleAttribute {
    fn from_name(name: String) -> Option<V<ParticleAttribute>> {
        let attr = Attribute::from_name(&name)?;
        Some(ParticleAttribute { attr }.into())
    }
}

#[script_bindings(name = "particle_module_bindings", remote)]
impl ParticleEffectModule {
    fn new() -> V<ParticleEffectModule> {
        ParticleEffectModule::default().into()
    }

    fn attribute(
        mut module: M<ParticleEffectModule>,
        attribute: V<ParticleAttribute>,
    ) -> V<ParticleExprHandle> {
        ParticleExprHandle {
            expr: module.module.attr(attribute.into_inner().attr),
        }
        .into()
    }

    fn lit(mut module: M<ParticleEffectModule>, value: V<Value>) -> V<ParticleExprHandle> {
        ParticleExprHandle {
            expr: module.module.lit(*value),
        }
        .into()
    }

    fn lit_f32(
        _ctxt: FunctionCallContext,
        module: M<ParticleEffectModule>,
        value: f32,
    ) -> V<ParticleExprHandle> {
        let m = module.0;
        let expr = m.module.lit(value);
        ParticleExprHandle { expr }.into()
    }

    fn lit_vec2(
        _ctxt: FunctionCallContext,
        module: M<ParticleEffectModule>,
        value: V<Vec2>,
    ) -> V<ParticleExprHandle> {
        let m = module.0;
        let expr = m.module.lit(*value);
        ParticleExprHandle { expr }.into()
    }

    fn lit_vec3(module: M<ParticleEffectModule>, value: V<Vec3>) -> V<ParticleExprHandle> {
        let m = module.0;
        let expr = m.module.lit(*value);
        ParticleExprHandle { expr }.into()
    }

    fn lit_vec4(module: M<ParticleEffectModule>, value: V<Vec4>) -> V<ParticleExprHandle> {
        let m = module.0;
        let expr = m.module.lit(*value);
        ParticleExprHandle { expr }.into()
    }

    fn rgba_u32(
        module: M<ParticleEffectModule>,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    ) -> V<ParticleExprHandle> {
        let r = (r.clamp(0.0, 1.0) * 255.0) as u32;
        let g = (g.clamp(0.0, 1.0) * 255.0) as u32;
        let b = (b.clamp(0.0, 1.0) * 255.0) as u32;
        let a = (a.clamp(0.0, 1.0) * 255.0) as u32;

        let color = (r << 24) | (g << 16) | (b << 8) | a;
        let m = module.0;
        let expr = m.module.lit(color);
        ParticleExprHandle { expr }.into()
    }

    fn real_time(module: M<ParticleEffectModule>) -> V<ParticleExprHandle> {
        let m = module.0;
        ParticleExprHandle {
            expr: m.module.builtin(BuiltInOperator::RealTime),
        }
        .into()
    }

    fn real_delta_time(module: M<ParticleEffectModule>) -> V<ParticleExprHandle> {
        let m = module.0;

        ParticleExprHandle {
            expr: m.module.builtin(BuiltInOperator::RealDeltaTime),
        }
        .into()
    }

    fn virtual_time(module: M<ParticleEffectModule>) -> V<ParticleExprHandle> {
        let m = module.0;

        ParticleExprHandle {
            expr: m.module.builtin(BuiltInOperator::VirtualTime),
        }
        .into()
    }

    fn virtual_delta_time(module: M<ParticleEffectModule>) -> V<ParticleExprHandle> {
        let m = module.0;

        ParticleExprHandle {
            expr: m.module.builtin(BuiltInOperator::VirtualDeltaTime),
        }
        .into()
    }

    fn rand(module: M<ParticleEffectModule>, value: V<ValueType>) -> V<ParticleExprHandle> {
        let m = module.0;

        ParticleExprHandle {
            expr: m.module.builtin(BuiltInOperator::Rand(*value)),
        }
        .into()
    }

    fn add(
        _ctxt: FunctionCallContext,
        module: M<ParticleEffectModule>,
        a: V<ParticleExprHandle>,
        b: V<ParticleExprHandle>,
    ) -> Result<V<ParticleExprHandle>, InteropError> {
        let m = module.0;
        let expr = m.module.add(a.0.expr, b.0.expr);
        Ok(V::new(ParticleExprHandle { expr }))
    }

    fn mul(
        _ctxt: FunctionCallContext,
        module: M<ParticleEffectModule>,
        a: V<ParticleExprHandle>,
        b: V<ParticleExprHandle>,
    ) -> Result<V<ParticleExprHandle>, InteropError> {
        let m = module.0;
        let expr = m.module.mul(a.0.expr, b.0.expr);
        Ok(V::new(ParticleExprHandle { expr }))
    }
}

// -- Builder Api --

#[script_bindings(name = "particle_effect_builder", remote)]
impl ParticleEffectBuilder {
    fn new() -> V<ParticleEffectBuilder> {
        V::new(Default::default())
    }

    fn with_capacity(
        mut builder: V<ParticleEffectBuilder>,
        capacity: u32,
    ) -> V<ParticleEffectBuilder> {
        builder.0.capacity = Some(capacity);
        builder.0.into()
    }

    fn with_name(mut builder: V<ParticleEffectBuilder>, name: String) -> V<ParticleEffectBuilder> {
        builder.0.name = Some(name);
        builder.0.into()
    }

    /// The number of particles to spawn each spawn cycle
    fn with_spawner_particle_count(
        mut builder: V<ParticleEffectBuilder>,
        count: u32,
    ) -> V<ParticleEffectBuilder> {
        builder.0.spawn_cycle_count = Some(count);
        builder.0.into()
    }

    /// the number of spawn cycles, if set to 0, will be infinite
    fn with_spawner_cycle_count(
        mut builder: V<ParticleEffectBuilder>,
        count: u32,
    ) -> V<ParticleEffectBuilder> {
        builder.0.spawn_cycle_count = Some(count);
        builder.0.into()
    }

    /// How often to trigger a spawn period
    fn with_spawner_cycle_period(
        mut builder: V<ParticleEffectBuilder>,
        count: f32,
    ) -> V<ParticleEffectBuilder> {
        builder.0.spawn_cycle_period = Some(count);
        builder.0.into()
    }

    /// How long each spawn cycle lasts
    fn with_spawner_cycle_time(
        mut builder: V<ParticleEffectBuilder>,
        duration: f32,
    ) -> V<ParticleEffectBuilder> {
        builder.0.spawn_cycle_duration = Some(duration);
        builder.0.into()
    }

    fn with_module(
        mut builder: V<ParticleEffectBuilder>,
        module: V<ParticleEffectModule>,
    ) -> V<ParticleEffectBuilder> {
        builder.0.module = Some(module.into_inner());
        builder.0.into()
    }

    fn with_simulation_space(
        mut builder: V<ParticleEffectBuilder>,
        space: V<SimulationSpace>,
    ) -> V<ParticleEffectBuilder> {
        builder.0.simulation_space = Some(*space);
        builder.0.into()
    }

    fn with_simulation_condition(
        mut builder: V<ParticleEffectBuilder>,
        cond: V<SimulationCondition>,
    ) -> V<ParticleEffectBuilder> {
        builder.0.simulation_condition = Some(*cond);
        builder.0.into()
    }

    fn with_alpha_mode(
        mut builder: V<ParticleEffectBuilder>,
        mode: V<AlphaMode>,
    ) -> V<ParticleEffectBuilder> {
        builder.0.alpha_mode = Some(*mode);
        builder.0.into()
    }

    fn with_motion_integration(
        mut builder: V<ParticleEffectBuilder>,
        integration: V<MotionIntegration>,
    ) -> V<ParticleEffectBuilder> {
        builder.0.motion_integration = Some(*integration);
        builder.0.into()
    }

    fn set_mesh(
        mut builder: V<ParticleEffectBuilder>,
        mesh: V<ScriptHandleWrapper<Mesh>>,
    ) -> V<ParticleEffectBuilder> {
        builder.0.mesh = Some(mesh.0.0);
        builder.0.into()
    }

    fn init_modifier(
        mut builder: V<ParticleEffectBuilder>,
        modifier: V<ParticleModifier>,
    ) -> V<ParticleEffectBuilder> {
        builder.0.init_modifiers.push(modifier.0.inner);
        builder.0.into()
    }

    fn update_modifier(
        mut builder: V<ParticleEffectBuilder>,
        modifier: V<ParticleModifier>,
    ) -> V<ParticleEffectBuilder> {
        builder.0.update_modifiers.push(modifier.0.inner);
        builder.0.into()
    }

    fn render_modifier(
        mut builder: V<ParticleEffectBuilder>,
        modifier: V<ParticleRenderModifier>,
    ) -> V<ParticleEffectBuilder> {
        builder.0.render_modifiers.push(modifier.0.inner);
        builder.0.into()
    }

    fn build(
        ctxt: FunctionCallContext,
        builder: V<ParticleEffectBuilder>,
    ) -> Result<ScriptHandleWrapper<EffectAsset>, InteropError> {
        let world = ctxt.world()?;

        let handle = world.with_resource_mut(|mut asssets: Mut<Assets<EffectAsset>>| {
            asssets.add(builder.into_inner().build())
        })?;

        Ok(handle.into())
    }
}
