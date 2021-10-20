use crate::{configuration::sprites::player_sprite_scale, sprites::Sprites};

use super::{
    action::CurrentAction,
    movement::{Direction, Speed},
    tool::Tool,
};
use bevy::{
    math::{Vec2, Vec3},
    prelude::{Bundle, SpriteSheetBundle, Transform},
    sprite::TextureAtlasSprite,
};

pub struct PlayerName(pub String);

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

#[derive(Bundle)]
pub struct PlayerBundle {
    pub name: PlayerName,
    pub player_movement: PlayerMovement,
    pub player: Player,
    pub action: CurrentAction,
    pub inventory: PlayerInventory,

    #[bundle]
    pub sprite: SpriteSheetBundle,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        PlayerBundle {
            name: PlayerName("Goblin?!".to_string()),
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
            sprite: SpriteSheetBundle::default(),
        }
    }
}

impl PlayerBundle {
    pub fn build_main_player(coordinate: Vec2, sprites: &Sprites) -> Self {
        Self {
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
}
