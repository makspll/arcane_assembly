state = {
    sprite_fireball = world.load_asset_from_mod("Main", "sprites/fireball.ase"),
    sfx_fireball = world.load_asset_from_mod("Main", "audio/fireball_hit.wav")
}
function on_script_loaded()
end


function on_spell_cast(entity)
    entity:set_aseprite_animation(state.sprite_fireball, "Flying")

end



function on_spell_expired(entity)
    world.despawn(entity)
end

function on_spell_hit_character(entity, other_entity)
end


function on_spell_hit_terrain(entity, other_entity)
    world.play_sound_effect(state.sfx_fireball)
    world.despawn(entity)
end
