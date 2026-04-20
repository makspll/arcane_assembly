use bevy_mod_scripting::core::callback_labels;

callback_labels!(
    OnSpellCast => "on_spell_cast",
    OnSpellExpired => "on_spell_expired",
    OnSpellHitTerrain => "on_spell_hit_terrain",
    OnSpellHitCharacter => "on_spell_hit_character",
    OnAreaEffectTriggered => "on_spell_area_effect_triggered"
);
