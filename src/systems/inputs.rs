use bevy::{
    input::Input,
    math::Vec2,
    prelude::{KeyCode, Query, Res},
};

use crate::components::{player::Player, speed::Speed};

pub fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Speed)>,
) {
    let (_, mut speed) = query.single_mut().unwrap();

    speed.current = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::Left) {
        speed.current -= Vec2::new(5.0, 0.0);
    }

    if keyboard_input.pressed(KeyCode::Right) {
        speed.current += Vec2::new(5.0, 0.0);
    }

    if keyboard_input.pressed(KeyCode::Up) {
        speed.current += Vec2::new(0.0, 5.0);
    }

    if keyboard_input.pressed(KeyCode::Down) {
        speed.current -= Vec2::new(0.0, 5.0);
    }
}
