use std::usize;

use bevy::{
    math::Vec3,
    prelude::{
        AssetServer, Color, Commands, HorizontalAlign, OrthographicCameraBundle, Rect, Res, ResMut,
        SpriteSheetBundle, TextBundle, Transform, UiCameraBundle,
    },
    sprite::TextureAtlasSprite,
    text::{Text, TextAlignment, TextStyle},
    ui::{AlignSelf, PositionType, Style, Val},
};

use crate::{
    components::{
        camera::{GameCamera, UiCamera},
        grid::{Grid, GridBundle, GridCell, GridName},
        player::PlayerBundle,
    },
    configuration::map::{MAP_HEIGHT, MAP_WIDTH, TILE_SIZE},
    sprites::Sprites,
    states::GameState,
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
                font_size: 100.0,
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

    let mut cells: Vec<GridCell> = Vec::new();

    let left_x = -1 * TILE_SIZE as i32 * MAP_WIDTH;
    let right_x = TILE_SIZE as i32 * MAP_WIDTH;

    let bottom_y = -1 * TILE_SIZE as i32 * MAP_HEIGHT;
    let top_y = TILE_SIZE as i32 * MAP_HEIGHT;

    for x in (left_x..right_x).step_by(TILE_SIZE as usize) {
        for y in (bottom_y..top_y).step_by(TILE_SIZE as usize) {
            // commands.spawn_bundle(SpriteSheetBundle {
            //     transform: Transform {
            //         translation: Vec3::new(x as f32, y as f32, 0.0),
            //         scale: Vec3::splat(1.0),
            //         ..Default::default()
            //     },
            //     sprite: TextureAtlasSprite::new(sprites.outline_index as u32),
            //     texture_atlas: sprites.atlas_handle.clone(),
            //     ..Default::default()
            // });
            cells.push(GridCell {
                cell_size: TILE_SIZE as f32,
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
