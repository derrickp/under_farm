use bevy::{
    math::Vec2,
    prelude::{Entity, Mut, Query, Res, Transform, Visible, Without},
    render::camera::Camera,
    text::Text,
};

use crate::{
    components::{
        action::{CurrentAction, PickupAction},
        body::Body,
        bounding_box::BoundingBox,
        cameras::GameCamera,
        item::Item,
        movement::Direction,
        player::{Player, PlayerCoordinates, PlayerInventory, PlayerMovement},
        structure::Structure,
        text::PlayerStatsText,
    },
    configuration::{game::GameConfiguration, map::grid_coordinate_from_world},
};

type PlayerMovementQuery = (
    &'static Player,
    &'static PlayerMovement,
    &'static PlayerInventory,
    &'static mut Transform,
    &'static mut CurrentAction,
);

pub fn check_item_pickup(
    mut query: Query<(&Player, &Transform, &mut CurrentAction)>,
    item_query: Query<(&Item, &Body, &Visible, Entity)>,
) {
    let (_, transform, mut current_action): (&Player, &Transform, Mut<CurrentAction>) =
        query.single_mut();

    let x = transform.translation.x;
    let y = transform.translation.y;

    let bounding_box = BoundingBox::square(x, y, 60.0);

    for item_data in item_query.iter() {
        let (_, body, visible, entity): (&Item, &Body, &Visible, Entity) = item_data;

        if body.intersects_box(&bounding_box) && visible.is_visible && !body.underground {
            println!("picking up item");
            current_action.pickup = Some(PickupAction { target: entity })
        }
    }
}

pub fn player_movement(
    mut query: Query<PlayerMovementQuery>,
    cell_query: Query<(&Structure, &Body, Entity)>,
) {
    let (_, movement, inventory, mut transform, mut action): (
        &Player,
        &PlayerMovement,
        &PlayerInventory,
        Mut<Transform>,
        Mut<CurrentAction>,
    ) = query.single_mut();

    let x = movement.speed.current.x + transform.translation.x;
    let y = movement.speed.current.y + transform.translation.y;

    let bounding_box = BoundingBox::square(x, y, 60.0);

    let mut player_would_hit_wall: bool = false;

    for cell_data in cell_query.iter() {
        let (wall, body, entity): (&Structure, &Body, Entity) = cell_data;

        if body.intersects_box(&bounding_box) {
            if wall.can_be_walked_on() {
                continue;
            }

            if !wall.can_be_broken() && !wall.can_be_walked_on() {
                player_would_hit_wall = true;
                break;
            }

            if inventory.current_tool.is_none() {
                player_would_hit_wall = true;
                break;
            }

            let tool = inventory.current_tool.clone().unwrap();

            if let Some(damage) = tool.damage {
                let damage_done = damage.damage_dealt();
                action.hit_entity(damage_done, entity);
                player_would_hit_wall = true;
                break;
            }
        }
    }

    if player_would_hit_wall {
        return;
    }

    transform.translation.x = x;
    transform.translation.y = y;
}

pub fn update_player_grid_coordinate(
    mut query: Query<(&Player, &Transform, &mut PlayerCoordinates)>,
    game_config: Res<GameConfiguration>,
) {
    if query.is_empty() {
        return;
    }

    let (_, transform, mut player_coordinates): (&Player, &Transform, Mut<PlayerCoordinates>) =
        query.single_mut();

    let world_coordinate = Vec2::new(transform.translation.x, transform.translation.y);
    let current = grid_coordinate_from_world(
        &world_coordinate,
        game_config.map_size(),
        game_config.tile_size(),
    );
    player_coordinates.current = Some(current);
}

pub fn update_player_text(
    mut query: Query<(&PlayerStatsText, &mut Text)>,
    player_query: Query<&PlayerCoordinates>,
) {
    if query.is_empty() || player_query.is_empty() {
        return;
    }

    let (_, mut text): (&PlayerStatsText, Mut<Text>) = query.single_mut();
    let player_coordinates: &PlayerCoordinates = player_query.single();

    let section = text.sections.get_mut(0).unwrap();
    let current = player_coordinates.current.unwrap();
    section.value = format!("Coordinate {}  {}", current.x, current.y);
}

type GameCameraTransform = (&'static GameCamera, &'static Camera, &'static mut Transform);

pub fn camera_movement(
    player_query: Query<(&Player, &Transform, Without<Camera>)>,
    mut camera_query: Query<GameCameraTransform>,
) {
    if player_query.is_empty() || camera_query.is_empty() {
        return;
    }

    let (_, transform, _): (&Player, &Transform, bool) = player_query.single();
    let player_x = transform.translation.x;
    let player_y = transform.translation.y;

    let (_, _, mut camera_transform): (&GameCamera, &Camera, Mut<Transform>) =
        camera_query.single_mut();

    camera_transform.translation.x = player_x;
    camera_transform.translation.y = player_y;
}

pub fn check_floor_collision(
    player_query: Query<(&Player, &Transform, &PlayerMovement)>,
    mut ground_cell_query: Query<(&Body, &mut Visible)>,
    game_config: Res<GameConfiguration>,
) {
    let (_, transform, movement): (&Player, &Transform, &PlayerMovement) = player_query.single();

    let bounding_boxes = build_visibility_box(
        transform.translation.x,
        transform.translation.y,
        &movement.direction,
        game_config.tile_size(),
    );

    for cell_data in ground_cell_query.iter_mut() {
        let (grid_cell, mut visible): (&Body, Mut<Visible>) = cell_data;

        if bounding_boxes
            .iter()
            .any(|bounds| grid_cell.intersects_box(bounds))
            && !visible.is_visible
            && !grid_cell.underground
        {
            visible.is_visible = true;
        }
    }
}

fn build_visibility_box(x: f32, y: f32, direction: &Direction, tile_size: f32) -> Vec<BoundingBox> {
    let width = tile_size - 2.0;

    let player_box = BoundingBox::square(x, y, width);

    let mut visibility_boxes: Vec<BoundingBox> = match direction {
        Direction::North => vec![BoundingBox::square(x, y + tile_size, width)],
        Direction::South => vec![BoundingBox::square(x, y - tile_size, width)],
        Direction::East => vec![BoundingBox::square(x + tile_size, y, width)],
        Direction::West => vec![BoundingBox::square(x - tile_size, y, width)],
        Direction::NorthEast => vec![
            BoundingBox::square(x, y + tile_size, width),
            BoundingBox::square(x + tile_size, y, width),
            BoundingBox::square(x + tile_size, y + tile_size, width),
        ],
        Direction::NorthWest => vec![
            BoundingBox::square(x, y + tile_size, width),
            BoundingBox::square(x - tile_size, y, width),
            BoundingBox::square(x - tile_size, y + tile_size, width),
        ],
        Direction::SouthEast => vec![
            BoundingBox::square(x, y - tile_size, width),
            BoundingBox::square(x + tile_size, y, width),
            BoundingBox::square(x + tile_size, y - tile_size, width),
        ],
        Direction::SouthWest => vec![
            BoundingBox::square(x, y - tile_size, width),
            BoundingBox::square(x - tile_size, y, width),
            BoundingBox::square(x - tile_size, y - tile_size, width),
        ],
        _ => Vec::new(),
    };

    visibility_boxes.push(player_box);

    visibility_boxes
}
