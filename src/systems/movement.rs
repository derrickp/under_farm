use bevy::{
    prelude::{Mut, Query, QuerySet, Transform, Visible},
    render::camera::Camera,
};

use crate::{
    components::{
        camera::GameCamera,
        map::{BoundingBox, MapTile, WallTile},
        movement::Direction,
        player::{Player, PlayerMovement},
    },
    configuration::map::TILE_SIZE,
};

pub fn player_movement(
    mut query: Query<(&Player, &PlayerMovement, &mut Transform)>,
    cell_query: Query<(&WallTile, &MapTile)>,
) {
    let (_, movement, mut transform): (&Player, &PlayerMovement, Mut<'_, Transform>) =
        query.single_mut().unwrap();

    let x = movement.speed.current.x + transform.translation.x;
    let y = movement.speed.current.y + transform.translation.y;

    let bounding_box = BoundingBox::square(x, y, 60.0);

    let mut player_would_hit_wall: bool = false;

    for cell_data in cell_query.iter() {
        let (_, cell): (&WallTile, &MapTile) = cell_data;

        if cell.intersects_box(&bounding_box) {
            player_would_hit_wall = true;
            break;
        }
    }

    if player_would_hit_wall {
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
    player_query: Query<(&Player, &Transform, &PlayerMovement)>,
    mut ground_cell_query: Query<(&MapTile, &mut Visible)>,
) {
    let (_, transform, movement): (&Player, &Transform, &PlayerMovement) =
        player_query.single().unwrap();

    let bounding_boxes = build_visibility_box(
        transform.translation.x,
        transform.translation.y,
        &movement.direction,
    );

    for cell_data in ground_cell_query.iter_mut() {
        let (grid_cell, mut visible): (&MapTile, Mut<'_, Visible>) = cell_data;

        if bounding_boxes
            .iter()
            .any(|bounds| grid_cell.intersects_box(bounds))
            && !visible.is_visible
        {
            visible.is_visible = true;
        }
    }
}

fn build_visibility_box(x: f32, y: f32, direction: &Direction) -> Vec<BoundingBox> {
    let width = TILE_SIZE - 2.0;

    let player_box = BoundingBox::square(x, y, width);

    let mut visibility_boxes: Vec<BoundingBox> = match direction {
        Direction::North => vec![BoundingBox::square(x, y + TILE_SIZE, width)],
        Direction::South => vec![BoundingBox::square(x, y - TILE_SIZE, width)],
        Direction::East => vec![BoundingBox::square(x + TILE_SIZE, y, width)],
        Direction::West => vec![BoundingBox::square(x - TILE_SIZE, y, width)],
        Direction::NorthEast => vec![
            BoundingBox::square(x, y + TILE_SIZE, width),
            BoundingBox::square(x + TILE_SIZE, y, width),
            BoundingBox::square(x + TILE_SIZE, y + TILE_SIZE, width),
        ],
        Direction::NorthWest => vec![
            BoundingBox::square(x, y + TILE_SIZE, width),
            BoundingBox::square(x - TILE_SIZE, y, width),
            BoundingBox::square(x - TILE_SIZE, y + TILE_SIZE, width),
        ],
        Direction::SouthEast => vec![
            BoundingBox::square(x, y - TILE_SIZE, width),
            BoundingBox::square(x + TILE_SIZE, y, width),
            BoundingBox::square(x + TILE_SIZE, y - TILE_SIZE, width),
        ],
        Direction::SouthWest => vec![
            BoundingBox::square(x, y - TILE_SIZE, width),
            BoundingBox::square(x - TILE_SIZE, y, width),
            BoundingBox::square(x - TILE_SIZE, y - TILE_SIZE, width),
        ],
        _ => Vec::new(),
    };

    visibility_boxes.push(player_box);

    return visibility_boxes;
}
