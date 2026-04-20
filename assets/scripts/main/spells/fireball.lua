state = {
    sprite_fireball = world.load_asset_from_mod("Main", "sprites/fireball.ase"),
    sfx_fireball = world.load_asset_from_mod("Main", "audio/fireball_hit.wav")
}
function on_script_loaded()
    log_info("loaded plain ass projectile script")
end


function on_spell_cast(entity)
    entity:set_aseprite_animation(state.sprite_fireball, "Flying")

    log_info("plain ass projectile cast")
end


function on_spell_expired(entity)
    log_info("plain ass projectile expired")
    world.despawn(entity)
end

function on_spell_hit_character(entity, other_entity)
    log_info("plain ass projectile hit character")
end


function on_spell_hit_terrain(entity, other_entity)
    log_info("plain ass projectile hit terrain")
    world.play_sound_effect(state.sfx_fireball)
end
