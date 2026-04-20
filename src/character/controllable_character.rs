use crate::physics::{
    self, DOWN, GRAVITY_ACCELERATION_IN_METERS, GRAVITY_ACCELERATION_IN_PIXELS, LEFT,
    PIXELS_PER_METER, RIGHT, UP,
};
use bevy::{
    ecs::{
        bundle::Bundle,
        component::Component,
        message::MessageWriter,
        query::With,
        resource::Resource,
        system::{Local, Query, Res, ResMut},
    },
    input::{ButtonInput, keyboard::KeyCode, mouse::MouseButton},
    math::{Vec2, Vec3},
    prelude::{Deref, DerefMut},
    reflect::Reflect,
    time::{Fixed, Time},
    transform::components::Transform,
};
use bevy_mod_scripting::prelude::{ScriptCallbackEvent, ScriptComponent};
use bevy_rapier2d::prelude::{
    CharacterAutostep, CharacterLength, Collider, KinematicCharacterController,
    KinematicCharacterControllerOutput, RigidBody,
};

#[derive(Component, Reflect)]
pub struct Player;

/// A player is a character, but a character is not necessarily a player
#[derive(Component, Reflect)]
pub struct Character;

#[derive(Bundle)]
pub struct ControllableCharacter {
    marker: Character,
    pub scripts: ScriptComponent,
    pub controller: KinematicCharacterController,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub transform: Transform,
}

impl Default for ControllableCharacter {
    fn default() -> Self {
        Self {
            rigid_body: RigidBody::KinematicPositionBased,
            scripts: Default::default(),
            controller: KinematicCharacterController {
                up: physics::UP,
                offset: CharacterLength::Relative(0.08),
                snap_to_ground: Some(CharacterLength::Relative(0.2)),
                slide: true,
                autostep: Some(CharacterAutostep {
                    max_height: CharacterLength::Relative(0.3),
                    min_width: CharacterLength::Relative(0.5),
                    include_dynamic_bodies: false,
                }),
                ..Default::default()
            },
            // 1 meter diameter, should cover 32x32 sprite
            collider: Collider::ball(0.5),
            transform: Transform {
                translation: Vec3::ZERO,
                ..Default::default()
            },
            marker: Character,
        }
    }
}

/// Keyboard input vector
#[derive(Default, Resource, Deref, DerefMut)]
pub struct MovementInput(Vec2);

pub fn handle_input(keyboard: Res<ButtonInput<KeyCode>>, mut movement: ResMut<MovementInput>) {
    **movement = Vec2::ZERO;
    if keyboard.pressed(KeyCode::KeyW) {
        **movement += UP;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        **movement += DOWN;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        **movement += LEFT;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        **movement += RIGHT;
    }
    **movement = movement.normalize_or_zero();

    if keyboard.pressed(KeyCode::ShiftLeft) {
        **movement *= 2.0;
    }
}

const MOVEMENT_SPEED_IN_PIXELS: f32 = PIXELS_PER_METER * MOVEMENT_SPEED_IN_METERS;
const MOVEMENT_SPEED_IN_METERS: f32 = 1.0;

// TODO: this feels junk, the physics ain't adding up, make this feel "crunchy and buttery"
pub fn player_movement(
    time: Res<Time<Fixed>>,
    input: ResMut<MovementInput>,
    mut player: Query<
        (
            &mut KinematicCharacterController,
            Option<&KinematicCharacterControllerOutput>,
        ),
        With<Player>,
    >,
    mut vertical_velocity: Local<f32>,
) {
    let Ok((mut controller, output)) = player.single_mut() else {
        return;
    };
    let dt = time.delta_secs();
    let grounded = output.is_some_and(|o| o.grounded);

    if grounded && *vertical_velocity < 0.0 {
        *vertical_velocity = 0.0;
    }

    *vertical_velocity += GRAVITY_ACCELERATION_IN_METERS * dt;

    let horizontal = Vec2::new(input.x, 0.0) * MOVEMENT_SPEED_IN_METERS;

    let translation = Vec2::new(horizontal.x, *vertical_velocity) * dt;

    controller.translation = Some(translation);
}
