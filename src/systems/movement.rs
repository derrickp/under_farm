use bevy::{
    prelude::{Mut, Query, QuerySet, Transform, Visible},
    render::camera::Camera,
};

use crate::components::{
    camera::GameCamera,
    grid::{BoundingBox, GridCell},
    player::Player,
    speed::Speed,
};

pub fn player_movement(mut query: Query<(&Player, &Speed, &mut Transform)>, cell_query: Query<&GridCell>) {
    let (_, speed, mut transform): (&Player, &Speed, Mut<'_, Transform>) =
        query.single_mut().unwrap();

    let x = speed.current.x + transform.translation.x;
    let y = speed.current.y + transform.translation.y;

    let bounding_box = BoundingBox::square(x, y, 60.0);

    let mut player_in_grid: bool = false;

    for cell_data in cell_query.iter() {
        let cell: &GridCell = cell_data;

        if cell.intersects_box(&bounding_box) {
            player_in_grid = true;
            break;
        }
    }

    if !player_in_grid {
        return;
    }

    transform.translation.x = x;
    transform.translation.y = y;
}

pub fn camera_movement(
    mut query_set: QuerySet<(
        Query<(&Player, &Transform)>,
        Query<(&GameCamera, &Camera, &mut Transform)>,
    )>,
) {
    let mut player_x = 0.0;
    let mut player_y = 0.0;

    for e in query_set.q0_mut().iter_mut() {
        let (_, transform): (&Player, &Transform) = e;
        player_x = transform.translation.x;
        player_y = transform.translation.y;
    }

    for camera_data in query_set.q1_mut().iter_mut() {
        let (_, _, mut camera_transform): (&GameCamera, &Camera, Mut<'_, Transform>) = camera_data;

        camera_transform.translation.x = player_x;
        camera_transform.translation.y = player_y;
    }
}

pub fn check_floor_collision(
    player_query: Query<(&Player, &Transform)>,
    mut ground_cell_query: Query<(&GridCell, &mut Visible)>,
) {
    let (_, transform): (&Player, &Transform) = player_query.single().unwrap();

    let bounding_box = BoundingBox::square(transform.translation.x, transform.translation.y, 60.0);

    for cell_data in ground_cell_query.iter_mut() {
        let (grid_cell, mut visible): (&GridCell, Mut<'_, Visible>) = cell_data;

        if grid_cell.intersects_box(&bounding_box) && !visible.is_visible {
            visible.is_visible = true;
        }
    }
}
