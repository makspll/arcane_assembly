function on_script_loaded()
    print("loaded")
    local wizard_spritesheet_handle = world.load_asset_from_mod("Player", "sprites/wizard.ase")
    entity:set_aseprite_animation(wizard_spritesheet_handle, "Idle")
end