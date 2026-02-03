use bevy::{
    app::Plugin,
    state::{app::AppExtStates, state::States},
};

pub struct ArcaneAssemblyGameStatePlugin;

impl Plugin for ArcaneAssemblyGameStatePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_state::<GameState>();
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    CoreScriptsLoading,
    Running,
}
