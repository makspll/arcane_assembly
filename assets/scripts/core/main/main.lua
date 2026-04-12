local state = {}
function on_script_loaded()

    if not state.initialized then
        print("initializing core logic")

        print("iniitalized core logic")
        state.player_entity = player
        state.initialized = true
    end

end

function on_script_unloaded()
    return state
end

function on_script_reloaded(reloaded_state)

    state = reloaded_state
end

function on_update()

end
