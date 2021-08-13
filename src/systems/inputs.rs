use bevy::{
    core::{Time, Timer},
    input::Input,
    math::{Vec2, Vec3},
    prelude::{KeyCode, Mut, Query, Res, ResMut, State, Transform},
    render::camera::Camera,
};

use crate::{
    components::{action::Action, camera::GameCamera, player::Player, speed::Speed},
    configuration::map::TILE_SIZE,
    states::{AppState, GameState},
};

pub struct MovementInputTimer(pub Timer);

fn x_axis_speed() -> Vec2 {
    return Vec2::new(TILE_SIZE, 0.0);
}

fn y_axis_speed() -> Vec2 {
    return Vec2::new(0.0, TILE_SIZE);
}

pub fn movement_input_system(
    time: Res<Time>,
    mut timer: ResMut<MovementInputTimer>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Speed)>,
) {
    let (_, mut speed): (&Player, Mut<'_, Speed>) = query.single_mut().unwrap();

    speed.current = Vec2::ZERO;

    if keyboard_input.just_pressed(KeyCode::Left) {
        speed.current -= x_axis_speed();
    }

    if keyboard_input.just_pressed(KeyCode::Right) {
        speed.current += x_axis_speed();
    }

    if keyboard_input.just_pressed(KeyCode::Up) {
        speed.current += y_axis_speed();
    }

    if keyboard_input.just_pressed(KeyCode::Down) {
        speed.current -= y_axis_speed();
    }

    if speed.current.x != 0.0 || speed.current.y != 0.0 {
        timer.0.reset();
        return;
    }

    if timer.0.tick(time.delta()).just_finished() {
        if keyboard_input.pressed(KeyCode::Left) {
            speed.current -= x_axis_speed();
        }

        if keyboard_input.pressed(KeyCode::Right) {
            speed.current += x_axis_speed();
        }

        if keyboard_input.pressed(KeyCode::Up) {
            speed.current += y_axis_speed();
        }

        if keyboard_input.pressed(KeyCode::Down) {
            speed.current -= y_axis_speed();
        }
    }
}

pub fn action_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Action)>,
) {
    let (_, mut action): (&Player, Mut<'_, Action>) = query.single_mut().unwrap();

    if action.interact_pressed {
        action.interact_pressed = false;
    }

    if keyboard_input.just_pressed(KeyCode::E) {
        action.interact_pressed = true;
    }
}

pub fn open_close_inventory_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut state: ResMut<State<AppState>>,
) {
    if state.current().ne(&AppState::InGame) && state.current().ne(&AppState::InventoryScreen) {
        return;
    }

    if keyboard_input.just_pressed(KeyCode::I) {
        if state.current().eq(&AppState::InGame) {
            state.set(AppState::InventoryScreen).unwrap();
        } else {
            state.set(AppState::InGame).unwrap();
        }
    }
}

pub fn zoom_camera_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut game_state: ResMut<GameState>,
    mut query: Query<(&GameCamera, &Camera, &mut Transform)>,
) {
    if !keyboard_input.just_pressed(KeyCode::Z) {
        return;
    }

    for data in query.iter_mut() {
        let (_, _, mut transform): (&GameCamera, &Camera, Mut<'_, Transform>) = data;
        let new_scale = next_camera_scale(transform.scale);
        game_state.game_camera_scale = new_scale;
        transform.scale = new_scale;
    }
}

fn next_camera_scale(scale: Vec3) -> Vec3 {
    if scale == Vec3::splat(1.0) {
        return Vec3::new(2.0, 2.0, 1.0);
    }
    if scale == Vec3::new(2.0, 2.0, 1.0) {
        return Vec3::new(4.0, 4.0, 1.0);
    }

    return Vec3::splat(1.0);
}
