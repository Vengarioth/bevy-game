use bevy::prelude::*;

#[derive(Debug, Default)]
pub struct MovementData {
    pub acceleration: Vec3,
    pub velocity: Vec3,
}

pub fn apply_movement(
    mut entities: Query<(&mut MovementData, &mut Transform)>,
) {
    let speed = 30.0;
    for (mut movement_data, mut transform) in entities.iter_mut() {
        let mut position = transform.translation;
        let mut velocity = movement_data.velocity;
        let acceleration = movement_data.acceleration;

        position += velocity * 0.016;
        velocity += acceleration * 0.016 * speed;
        velocity *= 0.9;

        transform.translation = position;
        movement_data.velocity = velocity;
        movement_data.acceleration = Vec3::zero();
    }
}
