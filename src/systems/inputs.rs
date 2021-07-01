use bevy::{
    input::Input,
    prelude::{KeyCode, Query, Res},
};

use crate::{
    components::{player::Player, speed::Speed},
    game_state::GameState,
};

pub fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Speed)>,
    game_state: Res<GameState>,
) {
    if !game_state.map_loaded {
        return;
    }

    let (_, mut speed) = query.single_mut().unwrap();

    speed.x = 0;
    speed.y = 0;

    if keyboard_input.pressed(KeyCode::Left) {
        speed.x -= 5;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        speed.x += 5;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        speed.y += 5;
    }

    if keyboard_input.pressed(KeyCode::Down) {
        speed.y -= 5;
    }
}
