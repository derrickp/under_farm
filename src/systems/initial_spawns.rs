use std::usize;

use bevy::{
    math::{Vec2, Vec3},
    prelude::{Assets, Commands, OrthographicCameraBundle, Res, SpriteSheetBundle, Transform},
    sprite::{TextureAtlas, TextureAtlasSprite},
};

use crate::{
    components::{
        grid::{Grid, GridBundle, GridCell, GridName},
        player::{Player, PlayerBundle, PlayerName},
        speed::Speed,
    },
    sprite_handles::Sprites,
};

const TILE_SIZE: usize = 24;
const MAP_WIDTH: i32 = 100;
const MAP_HEIGHT: i32 = 100;

pub fn spawn_opening_bundles(
    mut commands: Commands,
    sprite_handles: Res<Sprites>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    let texture_atlas = texture_atlases.get(&sprite_handles.atlas_handle).unwrap();
    let sprite_index = texture_atlas
        .get_texture_index(&sprite_handles.player_sprite_handle)
        .unwrap();
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let mut cells: Vec<GridCell> = Vec::new();

    let left_x = -1 * TILE_SIZE as i32 * MAP_WIDTH;
    let right_x = TILE_SIZE as i32 * MAP_WIDTH;

    let bottom_y = -1 * TILE_SIZE as i32 * MAP_HEIGHT;
    let top_y = TILE_SIZE as i32 * MAP_HEIGHT;

    for x in (left_x..right_x).step_by(TILE_SIZE) {
        for y in (bottom_y..top_y).step_by(TILE_SIZE) {
            // let texture_atlas = texture_atlases.get(&sprite_handles.atlas_handle).unwrap();
            // let outline_index = texture_atlas
            //     .get_texture_index(&sprite_handles.outline_handle)
            //     .unwrap();
            // let entity_commands = commands.spawn_bundle(SpriteSheetBundle {
            //     transform: Transform {
            //         translation: Vec3::new(x as f32, y as f32, 0.0),
            //         scale: Vec3::splat(1.0),
            //         ..Default::default()
            //     },
            //     sprite: TextureAtlasSprite::new(outline_index as u32),
            //     texture_atlas: sprite_handles.atlas_handle.clone(),
            //     ..Default::default()
            // });
            cells.push(GridCell {
                cell_size: Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
                cell_center: Vec3::new(x as f32, y as f32, 0.0),
                contains_tile: false,
                sprite: None,
                outline: None,
            });
        }
    }

    commands.spawn_bundle(GridBundle {
        grid: Grid {
            cells,
            name: GridName("floor".to_string()),
        },
    });

    commands.spawn_bundle(PlayerBundle {
        name: PlayerName("Goblin?".to_string()),
        speed: Speed {
            current: Vec2::new(0.0, 0.0),
        },
        sprite: SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(sprite_index as u32),
            texture_atlas: sprite_handles.atlas_handle.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 5.0),
                scale: Vec3::splat(1.0),
                ..Default::default()
            },
            ..Default::default()
        },
        player: Player,
    });
}
