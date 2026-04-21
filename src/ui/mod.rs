use bevy::{
    app::{Plugin, Update},
    state::state::OnEnter,
};
use bevy_lunex::prelude::*;

use crate::{
    state::GameState,
    ui::interface::{
        GameUiColors, GameUiConfig, spawn_player_interface, sync_health_value_to_ui,
        sync_mana_value_to_ui,
    },
};

pub mod interface;

pub struct GameUiPlugin;
impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(UiLunexPlugins);
        app.init_resource::<GameUiConfig>();
        app.init_resource::<GameUiColors>();
        app.add_systems(OnEnter(GameState::Running), spawn_player_interface);
        app.add_systems(Update, (sync_mana_value_to_ui, sync_health_value_to_ui));
    }
}
