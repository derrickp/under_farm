use bevy::{
    core::{Time, Timer},
    input::Input,
    math::{Vec2, Vec3},
    prelude::{KeyCode, Mut, Query, Res, ResMut, State, Transform, Visible},
    render::camera::Camera,
};

use crate::{
    components::{
        action::CurrentAction,
        cameras::{GameCamera, GameCameraState},
        movement::Direction,
        player::{Player, PlayerMovement},
        text::PlayerStatsText,
    },
    configuration::{game::GameConfiguration, timers::movement_timer},
    states::AppState,
};

pub struct MovementInputTimer(pub Timer);

impl Default for MovementInputTimer {
    fn default() -> Self {
        Self(movement_timer())
    }
}

fn x_axis_speed(tile_size: f32) -> Vec2 {
    Vec2::new(tile_size, 0.0)
}

fn y_axis_speed(tile_size: f32) -> Vec2 {
    Vec2::new(0.0, tile_size)
}

pub fn movement_input_system(
    time: Res<Time>,
    mut timer: ResMut<MovementInputTimer>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut PlayerMovement)>,
    game_config: Res<GameConfiguration>,
) {
    let (_, mut movement): (&Player, Mut<PlayerMovement>) = query.single_mut();

    movement.speed.current = Vec2::ZERO;
    movement.direction = Direction::None;

    if keyboard_input.just_pressed(KeyCode::Left) {
        movement.speed.current -= x_axis_speed(game_config.tile_size());
        movement.direction = movement.direction + Direction::West;
    }

    if keyboard_input.just_pressed(KeyCode::Right) {
        movement.speed.current += x_axis_speed(game_config.tile_size());
        movement.direction = movement.direction + Direction::East;
    }

    if keyboard_input.just_pressed(KeyCode::Up) {
        movement.speed.current += y_axis_speed(game_config.tile_size());
        movement.direction = movement.direction + Direction::North;
    }

    if keyboard_input.just_pressed(KeyCode::Down) {
        movement.speed.current -= y_axis_speed(game_config.tile_size());
        movement.direction = movement.direction + Direction::South;
    }

    if movement.speed.current.x != 0.0 || movement.speed.current.y != 0.0 {
        timer.0.reset();
        return;
    }

    if timer.0.tick(time.delta()).just_finished() {
        if keyboard_input.pressed(KeyCode::Left) {
            movement.speed.current -= x_axis_speed(game_config.tile_size());
            movement.direction = movement.direction + Direction::West;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            movement.speed.current += x_axis_speed(game_config.tile_size());
            movement.direction = movement.direction + Direction::East;
        }

        if keyboard_input.pressed(KeyCode::Up) {
            movement.speed.current += y_axis_speed(game_config.tile_size());
            movement.direction = movement.direction + Direction::North;
        }

        if keyboard_input.pressed(KeyCode::Down) {
            movement.speed.current -= y_axis_speed(game_config.tile_size());
            movement.direction = movement.direction + Direction::South;
        }
    }
}

pub fn action_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut CurrentAction)>,
) {
    let (_, mut action): (&Player, Mut<CurrentAction>) = query.single_mut();

    if keyboard_input.just_pressed(KeyCode::E) {
        action.interact_pressed = true;
    }
}

pub fn reset_action_input_system(mut query: Query<(&Player, &mut CurrentAction)>) {
    let (_, mut action): (&Player, Mut<CurrentAction>) = query.single_mut();

    action.interact_pressed = false;
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

pub fn toggle_coordinates_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&PlayerStatsText, &mut Visible)>,
) {
    if !keyboard_input.just_pressed(KeyCode::Slash) {
        return;
    }

    if query.is_empty() {
        return;
    }

    let (_, mut visible): (&PlayerStatsText, Mut<Visible>) = query.single_mut();

    visible.is_visible = !visible.is_visible;
}

pub fn zoom_camera_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&GameCamera, &Camera, &mut Transform)>,
    mut camera_state_query: Query<&mut GameCameraState>,
) {
    if !keyboard_input.just_pressed(KeyCode::Z) {
        return;
    }

    if query.is_empty() {
        return;
    }

    let (_, _, mut transform): (&GameCamera, &Camera, Mut<Transform>) = query.single_mut();

    if camera_state_query.is_empty() {
        return;
    }

    let mut camera_state: Mut<GameCameraState> = camera_state_query.single_mut();

    let new_scale = next_camera_scale(transform.scale);
    camera_state.scale = new_scale;
    transform.scale = new_scale;
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
