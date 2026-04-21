use bevy::{
    camera::Camera,
    ecs::{
        query::With,
        reflect::ReflectResource,
        resource::Resource,
        system::{ResMut, Single},
    },
    math::Vec2,
    reflect::Reflect,
    transform::components::GlobalTransform,
    window::{PrimaryWindow, Window},
};

use crate::camera::MainCamera;

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct MousePositionInWorldCoordinates(pub Vec2);

pub fn compute_mouse_world_position(
    mut coords: ResMut<MousePositionInWorldCoordinates>,
    // query to get the window (so we can read the current cursor position)
    q_window: Single<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = *q_camera;
    let window = *q_window;

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_cursor_position) = window.cursor_position()
        && let Ok(ray) = camera.viewport_to_world(camera_transform, world_cursor_position)
    {
        let world_position = ray.origin.truncate();
        coords.0 = world_position;
    }
    // if left the window, keep last position of cursor
}
