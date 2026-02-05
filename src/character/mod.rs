use bevy::{
    app::{FixedUpdate, Plugin, PreUpdate, Startup},
    ecs::{schedule::IntoScheduleConfigs, system::Commands},
    input::InputSystems,
};

use crate::character::controllable_character::{
    ControllableCharacter, MovementInput, Player, handle_input, player_movement,
};
pub mod controllable_character;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_resource::<MovementInput>()
            .add_systems(Startup, spawn_dummy_character)
            .add_systems(PreUpdate, handle_input.after(InputSystems))
            .add_systems(FixedUpdate, player_movement);
        // .add_systems(FixedUpdate, );
    }
}

pub fn spawn_dummy_character(mut commands: Commands) {
    commands.spawn((ControllableCharacter::default(), Player));
}

// pub fn apply_gravity(mut characters: Query<&mut KinematicCharacterController>) {
//     for mut c in characters.iter_mut() {
//         c.translation = Some(DOWN * GRAVITY_ACCELERATION)
//     }
// }
