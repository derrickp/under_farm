use bevy::{
    math::{Vec2, Vec3},
    prelude::{Commands, Query, Res, SpriteSheetBundle, Transform},
    sprite::TextureAtlasSprite,
};

use crate::{
    components::{
        action::Action,
        crop::{Crop, CropBundle, CropName, CropStage, CropStages},
        map::BoundingBox,
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
                let stages: Vec<CropStage> = config
                    .stages
                    .iter()
                    .map(|stage| CropStage {
                        ticks_in_stage: 0,
                        min_ticks_in_stage: stage.min_ticks_in_stage,
                        chance_to_advance: stage.chance_to_advance,
                        sprite_index: stage.sprite_index.unwrap(),
                    })
                    .collect();
                spawn_crop(
                    Vec2::new(transform.translation.x, transform.translation.y),
                    commands,
                    sprites,
                    config.name,
                    stages,
                );
            }
        }
    }
}

fn spawn_crop(
    position: Vec2,
    mut commands: Commands,
    sprites: Res<Sprites>,
    crop_name: &str,
    stages: Vec<CropStage>,
) {
    commands.spawn_bundle(CropBundle {
        sprite: SpriteSheetBundle {
            transform: Transform {
                translation: Vec3::new(position.x, position.y, 2.0),
                scale: crate::configuration::sprites::sprite_scale(),
                ..Default::default()
            },
            sprite: TextureAtlasSprite::new(stages.get(0).unwrap().sprite_index),
            texture_atlas: sprites.atlas_handle.clone(),
            ..Default::default()
        },
        name: CropName(crop_name.to_string()),
        stages: CropStages { stages },
        crop: Crop {
            current_stage_index: 0,
        },
    });
}
