use bevy::prelude::{Query, Transform};

use crate::components::{player::Player, speed::Speed};

pub fn calculate_movement(mut query: Query<(&Player, &Speed, &mut Transform)>) {
    let (_, speed, mut transform) = query.single_mut().unwrap();

    let x = speed.x as f32 + transform.translation.x;
    let y = speed.y as f32 + transform.translation.y;

    transform.translation.x = x;
    transform.translation.y = y;
}
