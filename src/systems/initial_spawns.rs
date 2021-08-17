use std::usize;

use bevy::{
    math::Vec3,
    prelude::{
        AssetServer, Color, Commands, HorizontalAlign, OrthographicCameraBundle, Rect, Res, ResMut,
        SpriteSheetBundle, TextBundle, Transform, UiCameraBundle, Visible,
    },
    sprite::TextureAtlasSprite,
    text::{Text, TextAlignment, TextStyle},
    ui::{AlignSelf, PositionType, Style, Val},
};
use rand::Rng;

use crate::{
    components::{
        camera::{GameCamera, UiCamera},
        map::{GroundTile, GroundTileBundle, MapTile, WallTile, WallTileBundle},
        player::PlayerBundle,
    },
    configuration::map::{world_coordinate_from_grid, TILE_SIZE},
    sprites::Sprites,
    states::GameState,
    world_generation::generation::generate_world_grid,
};

pub fn spawn_opening_bundles(
    mut commands: Commands,
    sprites: Res<Sprites>,
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<GameState>,
) {
    if game_state.initial_spawn_complete {
        return;
    }

    let camera = commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(GameCamera)
        .id();
    game_state.game_camera = Some(camera);

    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(UiCamera);

    commands.spawn_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: Rect {
                bottom: Val::Px(5.0),
                right: Val::Px(15.0),
                ..Default::default()
            },
            ..Default::default()
        },
        // Use the `Text::with_section` constructor
        text: Text::with_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "under\nfarm!",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 50.0,
                color: Color::WHITE,
            },
            // Note: You can use `Default::default()` in place of the `TextAlignment`
            TextAlignment {
                horizontal: HorizontalAlign::Center,
                ..Default::default()
            },
        ),
        ..Default::default()
    });

    let grid = generate_world_grid();
    let mut rng = rand::thread_rng();

    for cell in grid.cells.values() {
        match cell.cell_type {
            crate::world_generation::grid::CellType::Floor => {
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
                let coordinate = world_coordinate_from_grid((cell.x, cell.y));
                let cell_center = Vec3::new(coordinate.0, coordinate.1, 0.0);
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
            crate::world_generation::grid::CellType::RoomWall => {
                let coordinate = world_coordinate_from_grid((cell.x, cell.y));
                let cell_center = Vec3::new(coordinate.0, coordinate.1, 0.0);
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
            crate::world_generation::grid::CellType::RoomFloor => {
                let coordinate = world_coordinate_from_grid((cell.x, cell.y));
                let cell_center = Vec3::new(coordinate.0, coordinate.1, 0.0);
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
            crate::world_generation::grid::CellType::Door => {
                let coordinate = world_coordinate_from_grid((cell.x, cell.y));
                let cell_center = Vec3::new(coordinate.0, coordinate.1, 0.0);
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
            crate::world_generation::grid::CellType::OuterWall => {
                let coordinate = world_coordinate_from_grid((cell.x, cell.y));
                let cell_center = Vec3::new(coordinate.0, coordinate.1, 0.0);

                commands.spawn_bundle(WallTileBundle {
                    cell_type: WallTile,
                    cell: MapTile {
                        cell_center: cell_center,
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

    commands.spawn_bundle(PlayerBundle {
        sprite: SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(sprites.player_sprite_index as u32),
            texture_atlas: sprites.atlas_handle.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 5.0),
                scale: crate::configuration::sprites::sprite_scale(),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });

    game_state.initial_spawn_complete = true;
}
