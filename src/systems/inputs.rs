use bevy::{
    input::Input,
    prelude::{KeyCode, Query, Res},
};

use crate::components::{player::Player, speed::Speed};

pub fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Speed)>,
) {
    let (_, mut speed) = query.single_mut().unwrap();

    let mut moving_left_or_right = false;
    let mut moving_up_or_down = false;

    if keyboard_input.pressed(KeyCode::Left) {
        moving_left_or_right = true;
        speed.current.x = -1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        moving_left_or_right = true;
        speed.current.x = 1.0;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        moving_up_or_down = true;
        speed.current.y = 1.0;
    }

    if keyboard_input.pressed(KeyCode::Down) {
        moving_up_or_down = true;
        speed.current.y = -1.0;
    }

    if !moving_left_or_right {
        speed.current.x = 0.0;
    }

    if !moving_up_or_down {
        speed.current.y = 0.0;
    }
}
