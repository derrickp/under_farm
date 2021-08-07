use bevy::{
    math::{Vec2, Vec3},
    prelude::{Commands, Query, Res, SpriteSheetBundle, Transform},
    sprite::TextureAtlasSprite,
};

use crate::{
    components::{
        action::Action,
        crop::{Crop, CropBundle, CropName},
        grid::BoundingBox,
        player::{Player, PlayerInventory},
    },
    configuration::crops::CropConfigurations,
    sprites::Sprites,
};

pub fn crop_actions(
    commands: Commands,
    crop_configurations: Res<CropConfigurations>,
    sprites: Res<Sprites>,
    query: Query<(&Player, &Action, &Transform, &PlayerInventory)>,
    crop_query: Query<(&Crop, &Transform)>,
) {
    let (_, action, transform, inventory): (&Player, &Action, &Transform, &PlayerInventory) =
        query.single().unwrap();
    let player_bounds = BoundingBox::square(
        transform.translation.x.floor(),
        transform.translation.y.floor(),
        60.0,
    );

    if action.interact_pressed {
        for crop_data in crop_query.iter() {
            let (_, crop_transform): (&Crop, &Transform) = crop_data;
            let crop_bounds = BoundingBox::square(
                crop_transform.translation.x.floor(),
                crop_transform.translation.y.floor(),
                60.0,
            );

            if crop_bounds.intersects(&player_bounds) {
                return;
            }
        }

        if let Some(config_index) = inventory.current_crop_selection {
            let config_result = crop_configurations
                .configurations
                .get(config_index as usize);
            if let Some(config) = config_result {
                if let Some(sprite_index) = config.sprite_index {
                    spawn_crop(
                        Vec2::new(transform.translation.x, transform.translation.y),
                        commands,
                        sprites,
                        sprite_index as u32,
                        config.name,
                    );
                }
            }
        }
    }
}

fn spawn_crop(
    position: Vec2,
    mut commands: Commands,
    sprites: Res<Sprites>,
    sprite_index: u32,
    crop_name: &str,
) {
    commands.spawn_bundle(CropBundle {
        sprite: SpriteSheetBundle {
            transform: Transform {
                translation: Vec3::new(position.x, position.y, 2.0),
                scale: Vec3::splat(2.0),
                ..Default::default()
            },
            sprite: TextureAtlasSprite::new(sprite_index),
            texture_atlas: sprites.atlas_handle.clone(),
            ..Default::default()
        },
        name: CropName(crop_name.to_string()),
        crop: Crop,
    });
}
