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
end

---@param entity Entity 
function on_spell_cast(entity)
    entity:set_aseprite_animation(main_fireball_state.sprite_fireball, "Flying")
end

function on_spell_expired(entity)
    world.despawn(entity)
end

function on_spell_hit_character(entity, other_entity)
    ---@type Health
    local health = world.get_component(other_entity, main_fireball_state.type_health_component)
    if health ~= nil then
        health.current = health.current - main_fireball_state.damage_per_hit
    end
    world.play_sound_effect(main_fireball_state.sfx_fireball)
    world.despawn(entity)
end


function on_spell_hit_terrain(entity, other_entity)

    world.play_sound_effect(main_fireball_state.sfx_fireball)
    world.despawn(entity)
end
