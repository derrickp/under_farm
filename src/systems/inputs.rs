use bevy::{
    core::{Time, Timer},
    input::Input,
    math::{Vec2, Vec3},
    prelude::{KeyCode, Mut, Query, Res, ResMut, Transform, Visible},
    render::camera::Camera,
};

use crate::{
    components::{
        action::{CurrentAction, InteractAction, PlantCropAction},
        body::Body,
        bounding_box::BoundingBox,
        cameras::{GameCamera, GameCameraState},
        movement::Direction,
        player::{Player, PlayerMovement},
        structure::Structure,
        text::PlayerStatsText,
    },
    configuration::{game::GameConfiguration, timers::movement_timer},
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
    mut query: Query<(&Player, &mut CurrentAction, &Transform)>,
    structure_query: Query<(&Structure, &Body)>,
) {
    let (_, mut action, transform): (&Player, Mut<CurrentAction>, &Transform) = query.single_mut();

    if keyboard_input.just_pressed(KeyCode::E) {
        let x = transform.translation.x;
        let y = transform.translation.y;
        let bounding_box = BoundingBox::square(x, y, 60.0);

        for structure_data in structure_query.iter() {
            let (structure, body): (&Structure, &Body) = structure_data;
            if structure.is_exit() && body.intersects_box(&bounding_box) {
                println!("Should drop");
                action.interact = Some(InteractAction::DropFloors);
                return;
            }
        }

        action.interact = Some(InteractAction::PlantCrop(PlantCropAction {
            position: Vec2::new(x, y),
        }));
    }
}

pub fn reset_action_input_system(mut query: Query<(&Player, &mut CurrentAction)>) {
    let (_, mut action): (&Player, Mut<CurrentAction>) = query.single_mut();

    action.interact = None;
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
