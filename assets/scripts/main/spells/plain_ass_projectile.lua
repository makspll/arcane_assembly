function on_script_loaded()
    log_info("loaded plain ass projectile script")
end


function on_spell_spawned()

end


function on_spell_expired(entity)
    log_info("plain ass projectile expired")
    world.despawn(entity)
end

