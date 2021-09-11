use tdlg::coordinate::Coordinate;

use super::sprites::{SPRITE_SCALE, SPRITE_SIZE};

pub const TILE_SIZE: f32 = SPRITE_SIZE * SPRITE_SCALE;
pub const MAP_WIDTH: i32 = 100;
pub const MAP_HEIGHT: i32 = 100;

pub fn world_coordinate_from_grid(grid_coordinate: &Coordinate<i32>) -> Coordinate<f32> {
    // conversion formula: tile_size * coordinate - (tile_size * (map_width / 2))
    let x = TILE_SIZE * grid_coordinate.x as f32 - (TILE_SIZE * (MAP_WIDTH / 2) as f32);
    let y = TILE_SIZE * grid_coordinate.y as f32 - (TILE_SIZE * (MAP_HEIGHT / 2) as f32);

    return Coordinate { x, y };
}
