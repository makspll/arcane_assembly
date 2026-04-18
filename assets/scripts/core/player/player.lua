local state = {}
local input_to_animation_map = {
    left = {"Walk", true, 1},
    right = {"Walk", false, 1},
    jump = {"Jump", false, 2},
    unknown = {"Idle", false, 0},
}

function on_script_loaded()
    state = {
        animation = "Idle",
        spritesheet = world.load_asset_from_mod("Player", "sprites/wizard.ase")
    }

    entity:set_aseprite_animation(state.spritesheet, state.animation)
end

function on_script_unloaded()
    return state
end

function on_script_reloaded(reloaded_state)
    state = reloaded_state
end

function on_player_input(inputs)
    local highest_priority_animation = "Idle"
    local flip_sprite_final = false
    local max_priority = -1
    for _, input in pairs(inputs) do
        local input_anim = input_to_animation_map[input]
        if input_anim ~= nil then
            local animation, flip_sprite, priority = unpack(input_anim)
            if priority > max_priority then
                highest_priority_animation = animation
                flip_sprite_final = flip_sprite
            end
        end

    end
    if highest_priority_animation ~= state.animation then
        state.animation = highest_priority_animation
        entity:set_aseprite_animation(state.spritesheet, state.animation, flip_sprite_final)
    end
end