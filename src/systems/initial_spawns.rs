use std::usize;

use bevy::{
    math::Vec3,
    prelude::{Commands, Res, ResMut, SpriteSheetBundle, Transform, Visible},
    sprite::TextureAtlasSprite,
};
use rand::Rng;
use tdlg::cell::CellType;

use crate::{
    components::{
        map::{GroundTile, GroundTileBundle, MapTile, WallTile, WallTileBundle},
        player::PlayerBundle,
    },
    configuration::map::{world_coordinate_from_grid, TILE_SIZE},
    sprites::Sprites,
    states::GameState,
    world_generation::grid::Grid,
};

pub fn spawn_opening_bundles(
    mut commands: Commands,
    sprites: Res<Sprites>,
    mut game_state: ResMut<GameState>,
    grid: Res<Grid>,
) {
    if game_state.initial_spawn_complete {
        return;
    }

    let mut rng = rand::thread_rng();

    for cell in grid.cells.values() {
        match cell.cell_type {
            CellType::Floor => {
                let random_index: usize = rng.gen_range(0..sprites.dirt_floor_indexes.len());
                let dirt_floor_index = sprites
                    .dirt_floor_indexes
                    .get(random_index)
                    .unwrap_or(
                        sprites
                            .dirt_floor_indexes
                            .first()
                            .expect("Need at least 1 floor sprite"),
                    )
                    .clone();
                let coordinate = world_coordinate_from_grid(&cell.coordinate);
                let cell_center = Vec3::new(coordinate.x, coordinate.y, 0.0);
                commands.spawn_bundle(GroundTileBundle {
                    cell_type: GroundTile,
                    cell: MapTile {
                        cell_center,
                        tile_size: TILE_SIZE as f32,
                        contains_tile: false,
                        sprite: None,
                        outline: None,
                    },
                    sprite: SpriteSheetBundle {
                        transform: Transform {
                            translation: cell_center,
                            scale: crate::configuration::sprites::sprite_scale(),
                            ..Default::default()
                        },
                        sprite: TextureAtlasSprite::new(dirt_floor_index as u32),
                        texture_atlas: sprites.atlas_handle.clone(),
                        visible: Visible {
                            is_visible: false,
                            is_transparent: false,
                        },
                        ..Default::default()
                    },
                });
            }
            CellType::RoomWall => {
                let coordinate = world_coordinate_from_grid(&cell.coordinate);
                let cell_center = Vec3::new(coordinate.x, coordinate.y, 0.0);
                commands.spawn_bundle(WallTileBundle {
                    cell_type: WallTile,
                    cell: MapTile {
                        cell_center,
                        tile_size: TILE_SIZE as f32,
                        contains_tile: false,
                        sprite: None,
                        outline: None,
                    },
                    sprite: SpriteSheetBundle {
                        transform: Transform {
                            translation: cell_center,
                            scale: crate::configuration::sprites::sprite_scale(),
                            ..Default::default()
                        },
                        sprite: TextureAtlasSprite::new(sprites.room_wall_index as u32),
                        texture_atlas: sprites.atlas_handle.clone(),
                        visible: Visible {
                            is_visible: true,
                            is_transparent: false,
                        },
                        ..Default::default()
                    },
                });
            }
            CellType::RoomFloor => {
                let coordinate = world_coordinate_from_grid(&cell.coordinate);
                let cell_center = Vec3::new(coordinate.x, coordinate.y, 0.0);
                commands.spawn_bundle(GroundTileBundle {
                    cell_type: GroundTile,
                    cell: MapTile {
                        cell_center,
                        tile_size: TILE_SIZE as f32,
                        contains_tile: false,
                        sprite: None,
                        outline: None,
                    },
                    sprite: SpriteSheetBundle {
                        transform: Transform {
                            translation: cell_center,
                            scale: crate::configuration::sprites::sprite_scale(),
                            ..Default::default()
                        },
                        sprite: TextureAtlasSprite::new(sprites.room_floor_index as u32),
                        texture_atlas: sprites.atlas_handle.clone(),
                        visible: Visible {
                            is_visible: false,
                            is_transparent: false,
                        },
                        ..Default::default()
                    },
                });
            }
            CellType::Door => {
                let coordinate = world_coordinate_from_grid(&cell.coordinate);
                let cell_center = Vec3::new(coordinate.x, coordinate.y, 0.0);
                commands.spawn_bundle(GroundTileBundle {
                    cell_type: GroundTile,
                    cell: MapTile {
                        cell_center,
                        tile_size: TILE_SIZE as f32,
                        contains_tile: false,
                        sprite: None,
                        outline: None,
                    },
                    sprite: SpriteSheetBundle {
                        transform: Transform {
                            translation: cell_center,
                            scale: crate::configuration::sprites::sprite_scale(),
                            ..Default::default()
                        },
                        sprite: TextureAtlasSprite::new(sprites.room_floor_index as u32),
                        texture_atlas: sprites.atlas_handle.clone(),
                        visible: Visible {
                            is_visible: false,
                            is_transparent: false,
                        },
                        ..Default::default()
                    },
                });
            }
            CellType::OuterWall => {
                let coordinate = world_coordinate_from_grid(&cell.coordinate);
                let cell_center = Vec3::new(coordinate.x, coordinate.y, 0.0);

                commands.spawn_bundle(WallTileBundle {
                    cell_type: WallTile,
                    cell: MapTile {
                        cell_center,
                        tile_size: TILE_SIZE as f32,
                        contains_tile: false,
                        sprite: None,
                        outline: None,
                    },
                    sprite: SpriteSheetBundle {
                        transform: Transform {
                            translation: cell_center,
                            scale: crate::configuration::sprites::sprite_scale(),
                            ..Default::default()
                        },
                        sprite: TextureAtlasSprite::new(sprites.outer_wall_index as u32),
                        texture_atlas: sprites.atlas_handle.clone(),
                        visible: Visible {
                            is_visible: true,
                            is_transparent: false,
                        },
                        ..Default::default()
                    },
                });
            }
            _ => {}
        }
    }

    let player_spawn = grid.random_spawnable_coordinate().unwrap();
    let coordinate = world_coordinate_from_grid(&player_spawn);
    commands.spawn_bundle(PlayerBundle {
        sprite: SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(sprites.player_sprite_index as u32),
            texture_atlas: sprites.atlas_handle.clone(),
            transform: Transform {
                translation: Vec3::new(coordinate.x, coordinate.y, 5.0),
                scale: crate::configuration::sprites::sprite_scale(),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });

    game_state.initial_spawn_complete = true;
}
