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
        player::Player,
    },
    sprites::Sprites,
};

pub fn crop_actions(
    commands: Commands,
    sprites: Res<Sprites>,
    query: Query<(&Player, &Action, &Transform)>,
    crop_query: Query<(&Crop, &Transform)>,
) {
    let (_, action, transform): (&Player, &Action, &Transform) = query.single().unwrap();
    let player_bounds = BoundingBox {
        min_x: transform.translation.x.floor() - 15.0,
        max_x: transform.translation.x.floor() + 15.0,
        min_y: transform.translation.y.floor() - 15.0,
        max_y: transform.translation.y.floor() + 15.0,
    };

    if action.interact_pressed {
        for crop_data in crop_query.iter() {
            let (_, crop_transform): (&Crop, &Transform) = crop_data;
            let crop_bounds = BoundingBox {
                min_x: crop_transform.translation.x.floor() - 15.0,
                max_x: crop_transform.translation.x.floor() + 15.0,
                min_y: crop_transform.translation.y.floor() - 15.0,
                max_y: crop_transform.translation.y.floor() + 15.0,
            };

            if crop_bounds.intersects(&player_bounds) {
                return;
            }
        }

        spawn_mushroom(
            Vec2::new(transform.translation.x, transform.translation.y),
            commands,
            sprites,
        );
    }
}

fn spawn_mushroom(position: Vec2, mut commands: Commands, sprites: Res<Sprites>) {
    commands.spawn_bundle(CropBundle {
        sprite: SpriteSheetBundle {
            transform: Transform {
                translation: Vec3::new(position.x, position.y, 2.0),
                scale: Vec3::splat(1.0),
                ..Default::default()
            },
            sprite: TextureAtlasSprite::new(sprites.mushroom_index as u32),
            texture_atlas: sprites.atlas_handle.clone(),
            ..Default::default()
        },
        name: CropName("mushroom".to_string()),
        crop: Crop,
    });
}
