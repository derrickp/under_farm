use bevy::{
    math::Vec3,
    prelude::{Commands, Query, Res, SpriteSheetBundle, Transform},
    sprite::TextureAtlasSprite,
};

use crate::{
    components::{action::Action, player::Player},
    sprites::Sprites,
};

pub fn crop_actions(
    mut commands: Commands,
    sprites: Res<Sprites>,
    mut query: Query<(&Player, &Action, &Transform)>,
) {
    let (_, action, transform): (&Player, &Action, &Transform) = query.single_mut().unwrap();

    if action.interact_pressed {
        commands.spawn_bundle(SpriteSheetBundle {
            transform: Transform {
                translation: Vec3::new(transform.translation.x, transform.translation.y, 2.0),
                scale: Vec3::splat(1.0),
                ..Default::default()
            },
            sprite: TextureAtlasSprite::new(sprites.mushroom_index as u32),
            texture_atlas: sprites.atlas_handle.clone(),
            ..Default::default()
        });
    }
}
