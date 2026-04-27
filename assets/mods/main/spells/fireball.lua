main_fireball_state = {
    damage_per_hit = 10,
    type_health_component = types.Health,
    type_velocity = types.Velocity,
    sprite_fireball = world.load_asset_from_mod("Main", "sprites/fireball.ase"),
    sfx_fireball = world.load_asset_from_mod("Main", "audio/fireball_hit.wav")
}

function on_script_loaded()
    register_callback("on_cast_main_fireball", on_spell_cast)
    register_callback("on_hit_character_main_fireball", on_spell_hit_character)
    register_callback("on_hit_terrain_main_fireball", on_spell_hit_terrain)
    register_callback("on_expired_main_fireball", on_spell_expired)

    -- Create effect builder
    local builder = ParticleEffectBuilder.new()

    local module = ParticleEffectModule.new()

    -- expressions
    local age = module:lit_f32(0.0)
    local zero = module:lit_vec3(Vec3.new(0.0, 0.0, 0.0))
    local radius = module:lit_f32(0.01)
    local speed = module:lit_f32(0.2)
    local surface_dimension = ShapeDimension.surface()
    local size = module:lit_f32(0.01)
    local lifetime = module:lit_f32(0.8)
    local roundness = module:lit_f32(2/3)

    local gravity = module:lit_vec3(Vec3.new(0.0, -9.8, 0.0))
    local drag = module:lit_f32(1.5)

    local axis_up = module:lit_vec3(Vec3.new(0.0, 0.0, 1.0))

    -- modifiers
    local init_age = ParticleModifier.set_age(age)
    local init_pos = ParticleModifier.position_circle(zero, radius, surface_dimension, axis_up)
    local init_vel = ParticleModifier.velocity_circle(zero, speed, axis_up)
    local init_life = ParticleModifier.set_lifetime(lifetime)
    local init_size = ParticleModifier.set_size(size)
    local round = ParticleRenderModifier.round(roundness)
    local color = ParticleRenderModifier.color(Vec4.new(255.0, 180.0, 30.0, 100.0), ColorBlendMode.add(), ColorBlendMask.rgba())

    local accel = ParticleModifier.accel(gravity)
    local drag_mod = ParticleModifier.linear_drag(drag)

    -- build
    local effect =
        builder
            :with_name("fireball_particles")
            :with_capacity(4000)
            :with_spawner_particle_count(10)
            :with_spawner_cycle_period(0.01)
            :with_spawner_cycle_time(0.1)
            :with_spawner_cycle_count(5)
            :with_module(module)
            :init_modifier(init_age)
            :init_modifier(init_pos)
            :init_modifier(init_vel)
            :init_modifier(init_life)
            :init_modifier(init_size)
            :render_modifier(round)
            :render_modifier(color)
            :update_modifier(accel)
            :update_modifier(drag_mod)
            :build()

    main_fireball_state.particle_effect = effect
    print(main_fireball_state.particle_effect)
end

---@param entity Entity 
function on_spell_cast(entity)
    entity:set_aseprite_animation(main_fireball_state.sprite_fireball, "Flying")
end

function on_spell_expired(entity)
    log_warn("despawning expired:" .. tostring(entity))
    world.despawn(entity)
end

--- Emits sounds and particles as appropriate
---@param entity Entity
function collision_effects(entity)
    world.play_sound_effect(main_fireball_state.sfx_fireball)
    ---@type Transform
    local transform = world.get_component(entity, types.Transform)
    world.spawn_particle_effect(main_fireball_state.particle_effect, transform.translation)
end

function on_spell_hit_character(entity, other_entity)
    ---@type Health
    local health = world.get_component(other_entity, main_fireball_state.type_health_component)
    if health ~= nil then
        health.current = health.current - main_fireball_state.damage_per_hit
    end
    collision_effects(entity)
    world.despawn(entity)
end


function on_spell_hit_terrain(entity, other_entity)
    collision_effects(entity)
    world.despawn(entity)
end
