use bevy::{core::{Time, Timer}, input::Input, math::Vec2, prelude::{KeyCode, Query, Res, ResMut}};

use crate::components::{player::Player, speed::Speed};

pub struct MovementInputTimer(pub Timer);

pub fn keyboard_input_system(
    time: Res<Time>,
    mut timer: ResMut<MovementInputTimer>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Speed)>,
) {
    let (_, mut speed) = query.single_mut().unwrap();

    speed.current = Vec2::ZERO;

    if keyboard_input.just_pressed(KeyCode::Left) {
        speed.current -= Vec2::new(32.0, 0.0);
    }

    if keyboard_input.just_pressed(KeyCode::Right) {
        speed.current += Vec2::new(32.0, 0.0);
    }

    if keyboard_input.just_pressed(KeyCode::Up) {
        speed.current += Vec2::new(0.0, 32.0);
    }

    if keyboard_input.just_pressed(KeyCode::Down) {
        speed.current -= Vec2::new(0.0, 32.0);
    }

    if speed.current.x != 0.0 || speed.current.y != 0.0 {
        timer.0.reset();
        return;
    }

    if timer.0.tick(time.delta()).just_finished() {
        if keyboard_input.pressed(KeyCode::Left) {
            speed.current -= Vec2::new(32.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Right) {
            speed.current += Vec2::new(32.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Up) {
            speed.current += Vec2::new(0.0, 32.0);
        }

        if keyboard_input.pressed(KeyCode::Down) {
            speed.current -= Vec2::new(0.0, 32.0);
        }
    }
}
