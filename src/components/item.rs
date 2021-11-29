use bevy::{
    math::Vec3,
    prelude::{Bundle, SpriteSheetBundle, Transform, Visible},
    sprite::TextureAtlasSprite,
};

use crate::{configuration::tools::ToolConfiguration, sprites::Sprites};

use super::body::Body;

pub enum ItemType {
    Tool(ToolConfiguration),
}

pub struct Item {
    pub item_type: ItemType,
}

#[derive(Bundle)]
pub struct ItemBundle {
    pub item: Item,
    pub body: Body,

    #[bundle]
    pub sprite: SpriteSheetBundle,
}

impl ItemBundle {
    pub fn build(
        position: Vec3,
        sprites: &Sprites,
        item_index: u32,
        sprite_scale: f32,
        tile_size: f32,
        underground: bool,
        item_type: ItemType,
    ) -> Self {
        Self {
            body: Body {
                tile_size,
                underground,
                cell_center: position.clone(),
            },
            sprite: SpriteSheetBundle {
                transform: Transform {
                    translation: position,
                    scale: Vec3::splat(sprite_scale),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite::new(item_index),
                texture_atlas: sprites.atlas_handle.clone(),
                visible: Visible {
                    is_visible: false,
                    is_transparent: true,
                },
                ..Default::default()
            },
            item: Item { item_type },
        }
    }
}
