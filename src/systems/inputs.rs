use bevy::{
    core::{Time, Timer},
    input::Input,
    math::Vec2,
    prelude::{KeyCode, Mut, Query, Res, ResMut},
};

use crate::components::{action::Action, player::Player, speed::Speed};

pub struct MovementInputTimer(pub Timer);

const SPEED: f32 = 64.0; // 32 px normal size and 2x scale

fn x_axis_speed() -> Vec2 {
    return Vec2::new(SPEED, 0.0);
}

fn y_axis_speed() -> Vec2 {
    return Vec2::new(0.0, SPEED);
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
