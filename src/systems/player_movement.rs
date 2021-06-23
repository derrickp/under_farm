use bevy::prelude::{Query, Transform};

use crate::components::{player::Player, speed::Speed};

pub fn calculate_movement(mut query: Query<(&Player, &Speed, &mut Transform)>) {
    let (_, speed, mut transform) = query.single_mut().unwrap();

    let x = speed.current.x + transform.translation.x;
    let y = speed.current.y + transform.translation.y;

    transform.translation.x = x;
    transform.translation.y = y;
}
