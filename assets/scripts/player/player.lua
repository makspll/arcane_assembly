local state = {}
local input_to_animation_map = {
    left = {"Walk", true, 1},
    right = {"Walk", false, 1},
    jump = {"Jump", false, 2},
    unknown = {"Idle", false, 0},
}
local STEP_SFX_PERIOD_SECONDS = 0.3

function on_script_loaded()
    state = {
        animation_last_sound_time = 0,
        animation = "Idle",
        animation_flip_x = false,
        spritesheet = world.load_asset_from_mod("Player", "sprites/wizard.ase"),
        sfx_crunch = world.load_asset_from_mod("Player", "audio/crunch.wav"),
        sfx_step = world.load_asset_from_mod("Player", "audio/step.wav"),
    }

    entity:set_aseprite_animation(state.spritesheet, state.animation)
end

function on_script_unloaded()
    return state
end

function on_script_reloaded(reloaded_state)
    state = reloaded_state
end

function on_update(dt, elapsed_seconds)
    if state.animation == "Walk" and (elapsed_seconds - state.animation_last_sound_time > STEP_SFX_PERIOD_SECONDS) then
        state.animation_last_sound_time = elapsed_seconds
        world.play_sound_effect(state.sfx_step)
    end
end

function on_player_input(inputs, elapsed_seconds)
    local highest_priority_animation = "Idle"
    local flip_sprite_final = state.animation_flip_x
    local max_priority = -1
    local spell_fired = false
    for _, input in pairs(inputs) do
        local input_anim = input_to_animation_map[input]
        if input_anim ~= nil then
            local animation, flip_sprite, priority = unpack(input_anim)
            if priority > max_priority then
                highest_priority_animation = animation
                flip_sprite_final = flip_sprite
            end
        elseif input == "fire" then
            spell_fired = true
        end

    end
    if highest_priority_animation ~= state.animation then
        state.animation = highest_priority_animation
        state.animation_flip_x = flip_sprite_final
        entity:set_aseprite_animation(state.spritesheet, state.animation, flip_sprite_final)
        if state.animation == "Jump" then
            world.play_sound_effect(state.sfx_crunch)
        end
    end

    if spell_fired then
        ---@type MousePositionInWorldCoordinates
        local mouse_pos = world.get_resource(types.MousePositionInWorldCoordinates)
        ---@type GlobalTransform
        local entity_transform =  world.get_component(entity, types.GlobalTransform)
        local entity_world_pos_2d = entity_transform:translation():truncate()
        local speed_m = 10
        world.spawn_spell("Main", "fireball", mouse_pos[1], Vec2.new(mouse_pos[1].x - entity_world_pos_2d.x, 2) * speed_m)
        world.play_sound_effect(state.sfx_crunch)
    end
end