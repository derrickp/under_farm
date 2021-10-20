use std::usize;

use bevy::{
    math::Vec3,
    prelude::{Commands, Res, ResMut, SpriteSheetBundle, Transform, Visible},
    sprite::TextureAtlasSprite,
};
use rand::Rng;
use tdlg::{
    cell::{Cell, CellLayerType},
    grid::Grid,
};

use crate::{
    components::{
        body::Body,
        ground::{GroundTile, GroundTileBundle},
        health::{Health, HealthTextureMap},
        player::PlayerBundle,
        structure::{Structure, StructureBundle, StructureType},
    },
    configuration::{
        map::{world_coordinate_from_grid, TILE_SIZE},
        sprites::player_sprite_scale,
    },
    sprites::Sprites,
    states::GameState,
};

pub fn spawn_opening_bundles(
    mut commands: Commands,
    sprites: Res<Sprites>,
    mut game_state: ResMut<GameState>,
    grid: Res<Grid<i32>>,
) {
    if game_state.initial_spawn_complete {
        return;
    }

    let default_dirt_floor_index = *sprites
        .dirt_floor_indexes
        .first()
        .expect("Need at least 1 floor sprite");

    for cell in grid.cells.values() {
        for layer in cell.layers.iter() {
            match *layer {
                CellLayerType::Floor => {
                    let floor_bundle =
                        get_floor_component(&sprites, default_dirt_floor_index, cell);
                    commands.spawn_bundle(floor_bundle);
                }
                CellLayerType::RoomWall => {
                    let floor_bundle =
                        get_floor_component(&sprites, default_dirt_floor_index, cell);
                    commands.spawn_bundle(floor_bundle);
                    let wall_bundle = get_room_wall_component(&sprites, cell);
                    commands.spawn_bundle(wall_bundle);
                }
                CellLayerType::RoomFloor => {
                    let floor_bundle = get_room_floor_component(&sprites, cell);
                    commands.spawn_bundle(floor_bundle);
                }
                CellLayerType::Door => {
                    let floor_bundle = get_room_floor_component(&sprites, cell);
                    commands.spawn_bundle(floor_bundle);
                }
                CellLayerType::OuterWall => {
                    let wall_bundle = get_outer_wall_component(&sprites, cell);
                    commands.spawn_bundle(wall_bundle);
                }
                CellLayerType::Rubble => {
                    let rubble = get_rubble_component(&sprites, cell);
                    commands.spawn_bundle(rubble);
                }
                CellLayerType::Table => {
                    let table = get_table_component(&sprites, cell);
                    commands.spawn_bundle(table);
                }
                _ => {}
            }
        }
    }

    let player_bundle = get_player_component(&sprites, &grid);
    commands.spawn_bundle(player_bundle);

    game_state.initial_spawn_complete = true;
}

fn get_rubble_component(sprites: &Sprites, cell: &Cell<i32>) -> StructureBundle {
    let coordinate = world_coordinate_from_grid(&cell.coordinate);
    let cell_center = Vec3::new(coordinate.x, coordinate.y, 4.0);
    StructureBundle {
        structure: Structure {
            can_be_broken: false,
            can_be_walked_on: true,
            ..Default::default()
        },
        body: Body {
            cell_center,
            tile_size: TILE_SIZE as f32,
            sprite: None,
            outline: None,
        },
        sprite: SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(sprites.broken_wall_index as u32),
            texture_atlas: sprites.atlas_handle.clone(),
            transform: Transform {
                translation: cell_center,
                scale: crate::configuration::sprites::sprite_scale(),
                ..Default::default()
            },
            ..Default::default()
        },
    }
}

fn get_table_component(sprites: &Sprites, cell: &Cell<i32>) -> StructureBundle {
    let coordinate = world_coordinate_from_grid(&cell.coordinate);
    let cell_center = Vec3::new(coordinate.x, coordinate.y, 4.0);
    StructureBundle {
        structure: Structure {
            health: Health::same_health(2),
            structure_type: StructureType::Table,
            can_be_broken: true,
            health_textures: vec![
                HealthTextureMap {
                    min_health: -99,
                    max_health: 0,
                    texture_index: sprites.broken_small_table,
                },
                HealthTextureMap {
                    min_health: 1,
                    max_health: 3,
                    texture_index: sprites.table_index,
                },
            ],
            ..Default::default()
        },
        body: Body {
            cell_center,
            tile_size: TILE_SIZE as f32,
            sprite: None,
            outline: None,
        },
        sprite: SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(sprites.table_index as u32),
            texture_atlas: sprites.atlas_handle.clone(),
            visible: Visible {
                is_visible: false,
                is_transparent: true,
            },
            transform: Transform {
                translation: cell_center,
                scale: crate::configuration::sprites::sprite_scale(),
                ..Default::default()
            },
            ..Default::default()
        },
    }
}

fn get_player_component(sprites: &Sprites, grid: &Grid<i32>) -> PlayerBundle {
    let player_spawn = grid.random_spawnable_coordinate().unwrap();
    let coordinate = world_coordinate_from_grid(&player_spawn);
    PlayerBundle {
        sprite: SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(sprites.player_sprite_index as u32),
            texture_atlas: sprites.atlas_handle.clone(),
            transform: Transform {
                translation: Vec3::new(coordinate.x, coordinate.y, 5.0),
                scale: player_sprite_scale(),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    }
}

fn get_outer_wall_component(sprites: &Sprites, cell: &Cell<i32>) -> StructureBundle {
    let coordinate = world_coordinate_from_grid(&cell.coordinate);
    let cell_center = Vec3::new(coordinate.x, coordinate.y, 0.0);
    StructureBundle {
        structure: Structure::default(),
        body: Body {
            cell_center,
            tile_size: TILE_SIZE as f32,
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
                is_transparent: true,
            },
            ..Default::default()
        },
    }
}

fn get_room_wall_component(sprites: &Sprites, cell: &Cell<i32>) -> StructureBundle {
    let coordinate = world_coordinate_from_grid(&cell.coordinate);
    let wall_cell_center = Vec3::new(coordinate.x, coordinate.y, 1.0);
    StructureBundle {
        structure: Structure {
            can_be_broken: true,
            can_be_walked_on: false,
            health: Health::same_health(3),
            structure_type: StructureType::Wall,
            health_textures: vec![
                HealthTextureMap {
                    min_health: -99,
                    max_health: 0,
                    texture_index: sprites.broken_wall_index,
                },
                HealthTextureMap {
                    min_health: 1,
                    max_health: 1,
                    texture_index: sprites.brick_wall_really_cracked_index,
                },
                HealthTextureMap {
                    min_health: 2,
                    max_health: 2,
                    texture_index: sprites.brick_wall_cracked_index,
                },
                HealthTextureMap {
                    min_health: 3,
                    max_health: 3,
                    texture_index: sprites.room_wall_index,
                },
            ],
        },
        body: Body {
            cell_center: wall_cell_center,
            tile_size: TILE_SIZE as f32,
            sprite: None,
            outline: None,
        },
        sprite: SpriteSheetBundle {
            transform: Transform {
                translation: wall_cell_center,
                scale: crate::configuration::sprites::sprite_scale(),
                ..Default::default()
            },
            sprite: TextureAtlasSprite::new(sprites.room_wall_index as u32),
            texture_atlas: sprites.atlas_handle.clone(),
            visible: Visible {
                is_visible: true,
                is_transparent: true,
            },
            ..Default::default()
        },
    }
}

fn get_room_floor_component(sprites: &Sprites, cell: &Cell<i32>) -> GroundTileBundle {
    let coordinate = world_coordinate_from_grid(&cell.coordinate);
    let cell_center = Vec3::new(coordinate.x, coordinate.y, 0.0);
    GroundTileBundle {
        tile_type: GroundTile,
        collide: Body {
            cell_center,
            tile_size: TILE_SIZE as f32,
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
    }
}

fn get_floor_component(
    sprites: &Sprites,
    default_dirt_floor_index: usize,
    cell: &Cell<i32>,
) -> GroundTileBundle {
    let mut rng = rand::thread_rng();
    let random_index: usize = rng.gen_range(0..sprites.dirt_floor_indexes.len());
    let dirt_floor_index = *sprites
        .dirt_floor_indexes
        .get(random_index)
        .unwrap_or(&default_dirt_floor_index);
    let coordinate = world_coordinate_from_grid(&cell.coordinate);
    let cell_center = Vec3::new(coordinate.x, coordinate.y, 0.0);
    GroundTileBundle {
        tile_type: GroundTile,
        collide: Body {
            cell_center,
            tile_size: TILE_SIZE as f32,
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
    }
}
