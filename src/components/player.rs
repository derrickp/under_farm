use crate::{
    configuration::{
        crops::CropConfiguration, game::GameConfiguration, map::grid_coordinate_from_world,
    },
    sprites::Sprites,
};

use super::{
    action::CurrentAction,
    movement::{Direction, Speed},
    name::Name,
    tool::Tool,
};
use bevy::{
    math::{Vec2, Vec3},
    prelude::{Bundle, SpriteSheetBundle, Transform},
    sprite::TextureAtlasSprite,
};
use tdlg::coordinate::Coordinate;

pub struct PlayerInventory {
    pub current_crop_config: Option<CropConfiguration>,
    pub current_tool: Option<Tool>,
    pub held_seeds: Vec<CropConfiguration>,
}

pub struct Player;

pub struct PlayerMovement {
    pub speed: Speed,
    pub direction: Direction,
}

#[derive(Default)]
pub struct PlayerCoordinates {
    pub current: Option<Coordinate>,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub name: Name,
    pub player_movement: PlayerMovement,
    pub player: Player,
    pub action: CurrentAction,
    pub inventory: PlayerInventory,
    pub coordinates: PlayerCoordinates,

    #[bundle]
    pub sprite: SpriteSheetBundle,
}

impl PlayerBundle {
    pub fn build_main_player(
        coordinate: Vec2,
        sprites: &Sprites,
        config: &GameConfiguration,
    ) -> Self {
        let held_seeds: Vec<CropConfiguration> = config
            .crops_config
            .configurations
            .iter()
            .filter(|crop_config| crop_config.starter)
            .cloned()
            .collect();
        Self {
            name: Name(config.player_config.info.name.clone()),
            coordinates: PlayerCoordinates {
                current: Some(grid_coordinate_from_world(
                    &coordinate,
                    config.map_size(),
                    config.tile_size(),
                )),
            },
            sprite: SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(
                    config.player_config.starting_sprite().sprite_index.unwrap(),
                ),
                texture_atlas: sprites.atlas_handle.clone(),
                transform: Transform {
                    translation: Vec3::new(coordinate.x, coordinate.y, 5.0),
                    scale: Vec3::splat(config.sprite_config.player_scale),
                    ..Default::default()
                },
                ..Default::default()
            },
            player_movement: PlayerMovement {
                direction: Direction::None,
                speed: Speed::default(),
            },
            player: Player,
            action: CurrentAction::default(),
            inventory: PlayerInventory {
                held_seeds,
                current_crop_config: None,
                current_tool: None,
            },
        }
    }
}
