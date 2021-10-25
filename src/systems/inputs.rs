use bevy::{
    core::{Time, Timer},
    input::Input,
    math::{Vec2, Vec3},
    prelude::{KeyCode, Mut, Query, Res, ResMut, State, Transform},
    render::camera::Camera,
};

use crate::{
    components::{
        action::CurrentAction,
        cameras::GameCamera,
        movement::Direction,
        player::{Player, PlayerMovement},
    },
    configuration::{map::TILE_SIZE, timers::movement_timer},
    states::{AppState, GameState},
};

pub struct MovementInputTimer(pub Timer);

impl Default for MovementInputTimer {
    fn default() -> Self {
        Self(movement_timer())
    }
}

fn x_axis_speed() -> Vec2 {
    Vec2::new(TILE_SIZE, 0.0)
}

fn y_axis_speed() -> Vec2 {
    Vec2::new(0.0, TILE_SIZE)
}

pub fn movement_input_system(
    time: Res<Time>,
    mut timer: ResMut<MovementInputTimer>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut PlayerMovement)>,
) {
    let (_, mut movement): (&Player, Mut<'_, PlayerMovement>) = query.single_mut().unwrap();

    movement.speed.current = Vec2::ZERO;
    movement.direction = Direction::None;

    if keyboard_input.just_pressed(KeyCode::Left) {
        movement.speed.current -= x_axis_speed();
        movement.direction = movement.direction + Direction::West;
    }

    if keyboard_input.just_pressed(KeyCode::Right) {
        movement.speed.current += x_axis_speed();
        movement.direction = movement.direction + Direction::East;
    }

    if keyboard_input.just_pressed(KeyCode::Up) {
        movement.speed.current += y_axis_speed();
        movement.direction = movement.direction + Direction::North;
    }

    if keyboard_input.just_pressed(KeyCode::Down) {
        movement.speed.current -= y_axis_speed();
        movement.direction = movement.direction + Direction::South;
    }

    if movement.speed.current.x != 0.0 || movement.speed.current.y != 0.0 {
        timer.0.reset();
        return;
    }

    if timer.0.tick(time.delta()).just_finished() {
        if keyboard_input.pressed(KeyCode::Left) {
            movement.speed.current -= x_axis_speed();
            movement.direction = movement.direction + Direction::West;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            movement.speed.current += x_axis_speed();
            movement.direction = movement.direction + Direction::East;
        }

        if keyboard_input.pressed(KeyCode::Up) {
            movement.speed.current += y_axis_speed();
            movement.direction = movement.direction + Direction::North;
        }

        if keyboard_input.pressed(KeyCode::Down) {
            movement.speed.current -= y_axis_speed();
            movement.direction = movement.direction + Direction::South;
        }
    }
}

pub fn action_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut CurrentAction)>,
) {
    let (_, mut action): (&Player, Mut<'_, CurrentAction>) = query.single_mut().unwrap();

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

    Vec3::splat(1.0)
}
