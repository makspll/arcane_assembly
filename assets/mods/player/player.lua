local state = {}
local input_to_animation_map = {
    left = {"Walk", true, 1},
    right = {"Walk", false, 1},
    jump = {"Jump", false, 2},
    unknown = {"Idle", false, 0},
}
local STEP_SFX_PERIOD_SECONDS = 0.3
local MIN_FIRE_DELAY_SECONDS = 0.16
local MANA_RECOVERY_PER_SECOND = 33
local SPELL_TEXT = [[
digraph {
    Start [label="fireball"]
    B [label="explosion"]

    Start -> B [label="on_hit_terrain"]
    Start -> B [label="on_hit_character"]
}
]]

function on_script_loaded()
    state = {
        spell_graph = nil,
        mana_component = world.get_component(entity, types.Mana),

        animation_last_sound_time = 0,
        fire_last_time = 0,
        animation = nil,
        animation_flip_x = false,
        spritesheet = world.load_asset_from_mod("Player", "sprites/wizard.ase"),
        sfx_crunch = world.load_asset_from_mod("Player", "audio/crunch.wav"),
        sfx_step = world.load_asset_from_mod("Player", "audio/step.wav"),
    }
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

    local new_mana = math.min(state.mana_component.current + (MANA_RECOVERY_PER_SECOND * dt), state.mana_component.maximum)
    state.mana_component.current = new_mana

    -- TODO on_game_started or something
    if state.animation == nil then
        entity:set_aseprite_animation(state.spritesheet, "Idle")
    end
end

function on_player_input(inputs, elapsed_seconds)
    -- TOOD: start callback
    if state.spell_graph == nil then
        state.spell_graph = world.parse_spell("Main", SPELL_TEXT)
        log_info(tostring(state.spell_graph))
    end

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

    if spell_fired and (elapsed_seconds - state.fire_last_time > MIN_FIRE_DELAY_SECONDS) then

        state.fire_last_time = elapsed_seconds

        ---@type MousePositionInWorldCoordinates
        local mouse_pos = world.get_resource(types.MousePositionInWorldCoordinates)
        ---@type GlobalTransform
        local entity_transform =  world.get_component(entity, types.GlobalTransform)
        local entity_world_pos_2d = entity_transform:translation():truncate()
        local speed_m = 10

        world.cast_spell(
            state.spell_graph,
            entity,
            mouse_pos[1],
            Vec2.new(mouse_pos[1].x - entity_world_pos_2d.x, 2) * speed_m
        )


    end
end