use bevy::{
    prelude::{Query, Res, Transform},
    render::camera::Camera,
};

use crate::{
    components::{player::Player, speed::Speed},
    game_state::GameState,
};

pub fn calculate_movement(
    game_state: Res<GameState>,
    mut player_query: Query<(&Player, &Speed, &mut Transform)>,
) {
    if !game_state.map_loaded {
        return;
    }

    let (_, speed, mut transform) = player_query.single_mut().unwrap();

    transform.translation.x = speed.x as f32 + transform.translation.x;
    transform.translation.y = speed.y as f32 + transform.translation.y;
}

pub fn camera_movement(
    game_state: Res<GameState>,
    mut speed_query: Query<(&Player, &Speed)>,
    mut camera_query: Query<(&Camera, &mut Transform)>,
) {
    if !game_state.map_loaded {
        return;
    }

    let (_, speed) = speed_query.single_mut().unwrap();
    let (_, mut transform) = camera_query.single_mut().unwrap();

    transform.translation.x = speed.x as f32 + transform.translation.x;
    transform.translation.y = speed.y as f32 + transform.translation.y;
}
