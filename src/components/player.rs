use crate::{
    configuration::{map::grid_coordinate_from_world, player::PlayerConfig},
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
    pub current_crop_selection: Option<usize>,
    pub current_tool: Option<Tool>,
    pub current_tool_selection: Option<usize>,
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
        config: &PlayerConfig,
        map_size: usize,
        tile_size: f32,
        player_sprite_scale: f32,
    ) -> Self {
        Self {
            name: Name(config.info.name.clone()),
            coordinates: PlayerCoordinates {
                current: Some(grid_coordinate_from_world(&coordinate, map_size, tile_size)),
            },
            sprite: SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(config.starting_sprite().sprite_index.unwrap()),
                texture_atlas: sprites.atlas_handle.clone(),
                transform: Transform {
                    translation: Vec3::new(coordinate.x, coordinate.y, 5.0),
                    scale: Vec3::splat(player_sprite_scale),
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
                current_crop_selection: None,
                current_tool: None,
                current_tool_selection: None,
            },
        }
    }
}
