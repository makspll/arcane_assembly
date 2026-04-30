main_explosion_state = {
    damage_per_hit = 50,
    radius = 1
}

function make_particle_effect()
  -- Create effect builder
    local builder = ParticleEffectBuilder.new()

    local module = ParticleEffectModule.new()

    -- expressions
    local age = module:lit_f32(0.0)
    local zero = module:lit_vec3(Vec3.new(0.0, 0.0, 0.0))
    local radius = module:lit_f32(main_explosion_state.radius * 0.7)
    local age_attr = module:attribute(ParticleAttribute.age())
    local lifetime_attr = module:attribute(ParticleAttribute.lifetime())

    -- random firey like colour
    local gold = module:lit_vec4(Vec4.new(0.741, 0.729, 0.157, 0.0))
    local red  = module:lit_vec4(Vec4.new(0.8, 0.1, 0.05, 0.0))
    local t = module:rand(ValueType.float())
    local color_vec4 = module:mix(gold, red, t)

    local age_percentage = module:div(age_attr, lifetime_attr)
    local age_remaining = module:sub(module:lit_f32(1), age_percentage)
    local alpha = module:mul(module:lit_vec4(Vec4.new(0,0,0,1)), age_remaining)
    color = module:add(color_vec4, alpha)
    color = module:pack4x8snorm(color)

    -- speed and shape
    local speed = module:lit_f32(2)
    local surface_dimension = ShapeDimension.volume()
    local size = module:lit_f32(0.05)
    local lifetime = module:lit_f32(0.8)
    local roundness = module:lit_f32(0.4)

    -- local gravity = module:lit_vec3(Vec3.new(0.0, -9.8, 0.0))
    local drag = module:rand(ValueType.float())
    drag = module:mul(drag, module:lit_f32(15))

    local axis_up = module:lit_vec3(Vec3.new(0.0, 0.0, 1.0))

    -- modifiers
    local init_age = ParticleModifier.set_age(age)
    local init_pos = ParticleModifier.position_circle(zero, radius, surface_dimension, axis_up)
    local init_vel = ParticleModifier.velocity_circle(zero, speed, axis_up)
    local init_life = ParticleModifier.set_lifetime(lifetime)
    local init_size = ParticleModifier.set_size(size)
    local update_color = ParticleModifier.set_color(color)
    local round = ParticleRenderModifier.round(roundness)

    -- local accel = ParticleModifier.accel(gravity)
    local drag_mod = ParticleModifier.linear_drag(drag)

    -- build
    local effect =
        builder
            :with_capacity(100)
            :with_spawner_particle_count(20)
            :with_spawner_cycle_period(0)
            :with_spawner_cycle_time(0.1)
            :with_spawner_cycle_count(1)
            :with_module(module)
            :init_modifier(init_age)
            :init_modifier(init_pos)
            :init_modifier(init_vel)
            :init_modifier(init_life)
            :init_modifier(init_size)
            :update_modifier(update_color)
            :render_modifier(round)
            :update_modifier(drag_mod)
            :build()

    return effect
end

function on_script_loaded()
    register_callback("on_cast_main_explosion", on_spell_cast)
    -- register_callback("on_hit_character_main_explosion", on_spell_hit_character)
    -- register_callback("on_hit_terrain_main_explosion", on_spell_hit_terrain)
    -- register_callback("on_expired_main_explosion", on_spell_expired)

  
    main_explosion_state.particle_effect = make_particle_effect()
end


---@param entity Entity 
function on_spell_cast(entity)
    ---@type Transform
    local transform = world.get_component(entity, types.Transform)
    local position = transform.translation
    local entities_hit = world.circle_collision_query(position, main_explosion_state.radius)
    world.spawn_particle_effect_one_shot(main_explosion_state.particle_effect, position, 1)
    for i,hit_entity in pairs(entities_hit) do
        ---@type Health?
        local health = world.get_component(hit_entity, types.Health)
        if health ~= nil then
            health.current = health.current - main_explosion_state.damage_per_hit
        end
    end
end

-- function on_spell_expired(entity)
-- end

-- --- Emits sounds and particles as appropriate
-- ---@param entity Entity
-- function collision_effects(entity)
--     world.play_sound_effect(main_explosion_state.sfx_fireball)
-- end

-- function on_spell_hit_character(entity, other_entity)
--     ---@type Health
--     local health = world.get_component(other_entity, main_explosion_state.type_health_component)
--     if health ~= nil then
--         health.current = health.current - main_explosion_state.damage_per_hit
--     end
--     collision_effects(entity)
-- end

