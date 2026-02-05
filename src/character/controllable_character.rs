use bevy::{
    color::Color,
    ecs::{
        bundle::Bundle,
        component::Component,
        query::With,
        resource::Resource,
        system::{Local, Query, Res, ResMut},
    },
    input::{ButtonInput, keyboard::KeyCode},
    math::{Vec2, Vec3},
    prelude::{Deref, DerefMut},
    sprite::Sprite,
    time::{Fixed, Time},
    transform::components::Transform,
};
use bevy_rapier2d::prelude::{
    CharacterAutostep, CharacterLength, Collider, KinematicCharacterController,
    KinematicCharacterControllerOutput, RigidBody,
};

use crate::physics::{self, DOWN, GRAVITY_ACCELERATION, LEFT, PIXELS_PER_METER, RIGHT, UP};

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct ControllableCharacter {
    pub controller: KinematicCharacterController,
    pub collider: Collider,
    pub sprite: Sprite,
    pub transform: Transform,
}

impl Default for ControllableCharacter {
    fn default() -> Self {
        Self {
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
            collider: Collider::ball(5.0),
            sprite: Sprite::from_color(
                Color::WHITE,
                Vec2 {
                    x: PIXELS_PER_METER,
                    y: PIXELS_PER_METER,
                },
            ),
            transform: Transform {
                translation: Vec3::ZERO,
                ..Default::default()
            },
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

const MOVEMENT_SPEED: f32 = 50.0;

pub fn player_movement(
    time: Res<Time<Fixed>>,
    mut input: ResMut<MovementInput>,
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

    *vertical_velocity += GRAVITY_ACCELERATION * dt;

    let horizontal = Vec2::new(input.x, 0.0) * MOVEMENT_SPEED;

    let translation = Vec2::new(horizontal.x, *vertical_velocity) * dt;

    controller.translation = Some(translation);
}
