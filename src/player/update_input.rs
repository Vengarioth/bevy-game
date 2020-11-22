use bevy::{input::mouse::MouseMotion, prelude::*};
use crate::{
    player::Player,
    base::MovementData,
};

#[derive(Default)]
pub struct InputState {
    pub mouse_motion_event_reader: EventReader<MouseMotion>,
}

#[derive(Debug, Default)]
pub struct LookDirection {
    pub yaw: f32,
    pub pitch: f32,
}

pub fn update_player_look_direction(
    mut state: Local<InputState>,
    mouse_motion_events: Res<Events<MouseMotion>>,
    mut player_query: Query<(&Player, &mut LookDirection)>,
) {
    let mut mouse_delta = Vec2::default();
    for event in state.mouse_motion_event_reader.iter(&mouse_motion_events) {
        mouse_delta += event.delta * 0.016 * 0.1;
    }

    for (_, mut look_direction) in player_query.iter_mut() {
        look_direction.yaw += mouse_delta.x();
        look_direction.pitch += mouse_delta.y();
    }
}

pub fn update_player_acceleration(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&Player, &LookDirection, &mut MovementData)>,
) {
    let mut acceleration = Vec3::default();
    if keyboard_input.pressed(KeyCode::W) {
         acceleration += Vec3::new(0.0, 0.0, 1.0);
    }
    if keyboard_input.pressed(KeyCode::A) {
         acceleration += Vec3::new(1.0, 0.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::S) {
         acceleration += Vec3::new(0.0, 0.0, -1.0);
    }
    if keyboard_input.pressed(KeyCode::D) {
         acceleration += Vec3::new(-1.0, 0.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::Space) {
         acceleration += Vec3::new(0.0, 1.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::LShift) {
         acceleration += Vec3::new(0.0, -1.0, 0.0);
    }

    for (_player, look_direction, mut movement_data) in player_query.iter_mut() {
        let sin = look_direction.yaw.sin();
        let cos = look_direction.yaw.cos();

        let acceleration = Vec3::new(
            acceleration.x() * cos - acceleration.z() * sin,
            acceleration.y(),
            acceleration.z() * cos + acceleration.x() * sin,
        );

        movement_data.acceleration += acceleration;
    }
}
