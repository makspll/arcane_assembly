use std::{f32, sync::Arc};

use bevy::{
    asset::{Assets, Handle},
    color::LinearRgba,
    ecs::{
        entity::{ContainsEntity, Entity},
        hierarchy::ChildOf,
        world::{Mut, World},
    },
    math::{Vec2, Vec3, Vec4},
    mesh::Mesh,
    reflect::Reflect,
    time::{Time, Virtual},
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
    spells::spell::WithLifetime,
};

// -- Globals --
#[script_bindings(name = "global_particle_functions", remote, unregistered)]
impl World {
    /// Spawns a one-shot particle effect at a position
    fn spawn_particle_effect_one_shot(
        ctxt: FunctionCallContext,
        effect: V<ScriptHandleWrapper<EffectAsset>>,
        position: V<Vec3>,
        lifetime_seconds: f64,
    ) -> Result<V<Entity>, InteropError> {
        let world = ctxt.world()?;

        world.with_world_mut_access(|w| {
            let time = w.get_resource_ref::<Time<Virtual>>().expect("time missing");
            let with_lifetime = WithLifetime::new(&time, lifetime_seconds);
            let entity = w
                .commands()
                .spawn((
                    ParticleEffect::new(effect.0.0),
                    Transform::from_translation(*position),
                    with_lifetime,
                ))
                .id();

            w.flush();

            Ok(V::new(entity))
        })?
    }

    /// Spawns a persistent particle effect at an entity
    fn spawn_particle_effect(
        ctxt: FunctionCallContext,
        effect: V<ScriptHandleWrapper<EffectAsset>>,
        parent_entity: V<Entity>,
    ) -> Result<V<Entity>, InteropError> {
        let world = ctxt.world()?;

        world.with_world_mut_access(|w| {
            let entity = w
                .spawn((ChildOf(*parent_entity), ParticleEffect::new(effect.0.0)))
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

#[script_bindings(name = "particle_value_type_bindings", remote)]
impl ValueType {
    fn float() -> V<ValueType> {
        ValueType::Scalar(ScalarType::Float).into()
    }

    fn bool() -> V<ValueType> {
        ValueType::Scalar(ScalarType::Bool).into()
    }

    fn int() -> V<ValueType> {
        ValueType::Scalar(ScalarType::Int).into()
    }

    fn uint() -> V<ValueType> {
        ValueType::Scalar(ScalarType::Uint).into()
    }

    fn color() -> V<ValueType> {
        ValueType::Scalar(ScalarType::Uint).into()
    }

    fn vec2() -> V<ValueType> {
        ValueType::Vector(VectorType::VEC2F).into()
    }

    fn vec3() -> V<ValueType> {
        ValueType::Vector(VectorType::VEC3F).into()
    }

    fn vec4() -> V<ValueType> {
        ValueType::Vector(VectorType::VEC4F).into()
    }

    fn matrix(cols: u8, rows: u8) -> V<ValueType> {
        ValueType::Matrix(MatrixType::new(cols, rows)).into()
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

    fn color(v: V<LinearRgba>) -> V<Value> {
        Value::Scalar(ScalarValue::Uint(v.as_u32())).into()
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

    fn set_color(value: V<ParticleExprHandle>) -> V<ParticleModifier> {
        V::new(ParticleModifier {
            inner: Arc::new(SetAttributeModifier::new(Attribute::COLOR, value.0.expr)),
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

    ///     The age of the particle.
    /// Each time the particle is updated, the current simulation delta time is added to the particle's age. The age can be used to animate some other quantities; see the ColorOverLifetimeModifier for example.
    /// If the particle also has a lifetime (either a per-effect constant value, or a per-particle value stored in the Attribute::LIFETIME attribute), then when the age of the particle exceeds its lifetime, the particle dies and is not simulated nor rendered anymore.
    /// ScalarType::Float
    fn age() -> V<ParticleAttribute> {
        let attr = Attribute::AGE;
        ParticleAttribute { attr }.into()
    }

    /// The lifetime of the particle.
    /// This attribute stores a per-particle lifetime, which compared to the particle's age allows determining if the particle needs to be simulated and rendered. This requires the Attribute::AGE attribute to be used too.
    /// ScalarType::Float
    fn lifetime() -> V<ParticleAttribute> {
        let attr = Attribute::LIFETIME;
        ParticleAttribute { attr }.into()
    }

    /// The particle's base color.
    /// This attribute stores a per-particle color, which can be used for various purposes, generally as the base color for rendering the particle.
    /// ScalarType::Uint representing the RGBA components of the color encoded as 0xAABBGGRR, with a single byte per component, where the alpha value is stored in the most significant byte and the red value in the least significant byte. Note that this representation is the same as the one returned by LinearRgba::as_u32().
    fn color() -> V<ParticleAttribute> {
        let attr = Attribute::COLOR;
        ParticleAttribute { attr }.into()
    }

    /// The particle velocity in simulation space.
    /// VectorType::VEC3F representing the XYZ coordinates of the velocity.
    fn velocity() -> V<ParticleAttribute> {
        let attr = Attribute::VELOCITY;
        ParticleAttribute { attr }.into()
    }

    /// The particle position in simulation space.
    // VectorType::VEC3F representing the XYZ coordinates of the position.
    fn position() -> V<ParticleAttribute> {
        let attr = Attribute::POSITION;
        ParticleAttribute { attr }.into()
    }

    ///The particle's opacity (alpha).
    // This is a value in [0:1], where 0 corresponds to a fully transparent particle, and 1 to a fully opaque one.
    fn alpha() -> V<ParticleAttribute> {
        let attr = Attribute::ALPHA;
        ParticleAttribute { attr }.into()
    }

    /// The particle's uniform size.
    /// The particle is uniformly scaled by this size.
    /// ScalarType::Float
    fn size() -> V<ParticleAttribute> {
        let attr = Attribute::SIZE;
        ParticleAttribute { attr }.into()
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

    fn lit_u32(
        _ctxt: FunctionCallContext,
        module: M<ParticleEffectModule>,
        value: u32,
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

    fn lit_color(module: M<ParticleEffectModule>, v: V<LinearRgba>) -> V<ParticleExprHandle> {
        let m = module.0;

        let expr = m.module.lit(v.as_u32());
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

    fn mix(
        module: M<ParticleEffectModule>,
        a: V<ParticleExprHandle>,
        b: V<ParticleExprHandle>,
        fraction: V<ParticleExprHandle>,
    ) -> V<ParticleExprHandle> {
        let m = module.0;
        ParticleExprHandle {
            expr: m.module.mix(
                a.into_inner().expr,
                b.into_inner().expr,
                fraction.into_inner().expr,
            ),
        }
        .into()
    }

    fn clamp(
        module: M<ParticleEffectModule>,
        a: V<ParticleExprHandle>,
        b: V<ParticleExprHandle>,
        fraction: V<ParticleExprHandle>,
    ) -> V<ParticleExprHandle> {
        let m = module.0;
        ParticleExprHandle {
            expr: m.module.clamp(
                a.into_inner().expr,
                b.into_inner().expr,
                fraction.into_inner().expr,
            ),
        }
        .into()
    }

    fn smoothstep(
        module: M<ParticleEffectModule>,
        a: V<ParticleExprHandle>,
        b: V<ParticleExprHandle>,
        fraction: V<ParticleExprHandle>,
    ) -> V<ParticleExprHandle> {
        let m = module.0;
        ParticleExprHandle {
            expr: m.module.smoothstep(
                a.into_inner().expr,
                b.into_inner().expr,
                fraction.into_inner().expr,
            ),
        }
        .into()
    }

    fn vec3(
        module: M<ParticleEffectModule>,
        x: V<ParticleExprHandle>,
        y: V<ParticleExprHandle>,
        z: V<ParticleExprHandle>,
    ) -> V<ParticleExprHandle> {
        let m = module.0;
        ParticleExprHandle {
            expr: m.module.vec3(
                x.into_inner().expr,
                y.into_inner().expr,
                z.into_inner().expr,
            ),
        }
        .into()
    }

    fn x(module: M<ParticleEffectModule>, a: V<ParticleExprHandle>) -> V<ParticleExprHandle> {
        let m = module.0;
        let expr = m.module.x(a.0.expr);
        ParticleExprHandle { expr }.into()
    }

    fn y(module: M<ParticleEffectModule>, a: V<ParticleExprHandle>) -> V<ParticleExprHandle> {
        let m = module.0;
        let expr = m.module.y(a.0.expr);
        ParticleExprHandle { expr }.into()
    }

    fn z(module: M<ParticleEffectModule>, a: V<ParticleExprHandle>) -> V<ParticleExprHandle> {
        let m = module.0;
        let expr = m.module.z(a.0.expr);
        ParticleExprHandle { expr }.into()
    }

    fn pack4x8snorm(
        module: M<ParticleEffectModule>,
        a: V<ParticleExprHandle>,
    ) -> V<ParticleExprHandle> {
        let m = module.0;
        let expr = m.module.pack4x8snorm(a.0.expr);
        ParticleExprHandle { expr }.into()
    }

    fn cast(
        module: M<ParticleEffectModule>,
        a: V<ParticleExprHandle>,
        b: V<ValueType>,
    ) -> V<ParticleExprHandle> {
        let m = module.0;
        let expr = m.module.cast(a.0.expr, b.0);
        ParticleExprHandle { expr }.into()
    }

    // fn bitwise_and(
    //     module: M<ParticleEffectModule>,
    //     a: V<ParticleExprHandle>,
    //     b: V<ParticleExprHandle>,
    // ) -> Result<V<ParticleExprHandle>, InteropError> {
    //     let m = module.0;
    //     let expr = m.module.
    //     Ok(V::new(ParticleExprHandle { expr }))
    // }

    fn add(
        module: M<ParticleEffectModule>,
        a: V<ParticleExprHandle>,
        b: V<ParticleExprHandle>,
    ) -> Result<V<ParticleExprHandle>, InteropError> {
        let m = module.0;
        let expr = m.module.add(a.0.expr, b.0.expr);
        Ok(V::new(ParticleExprHandle { expr }))
    }

    fn sub(
        module: M<ParticleEffectModule>,
        a: V<ParticleExprHandle>,
        b: V<ParticleExprHandle>,
    ) -> Result<V<ParticleExprHandle>, InteropError> {
        let m = module.0;
        let expr = m.module.sub(a.0.expr, b.0.expr);
        Ok(V::new(ParticleExprHandle { expr }))
    }

    fn mul(
        module: M<ParticleEffectModule>,
        a: V<ParticleExprHandle>,
        b: V<ParticleExprHandle>,
    ) -> Result<V<ParticleExprHandle>, InteropError> {
        let m = module.0;
        let expr = m.module.mul(a.0.expr, b.0.expr);
        Ok(V::new(ParticleExprHandle { expr }))
    }

    fn div(
        module: M<ParticleEffectModule>,
        a: V<ParticleExprHandle>,
        b: V<ParticleExprHandle>,
    ) -> Result<V<ParticleExprHandle>, InteropError> {
        let m = module.0;
        let expr = m.module.div(a.0.expr, b.0.expr);
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
