use bevy::{
    math::{Vec2, Vec3},
    prelude::{Assets, Commands, OrthographicCameraBundle, Res, SpriteSheetBundle, Transform},
    sprite::{TextureAtlas, TextureAtlasSprite},
};

use crate::{
    components::{
        player::{Player, PlayerBundle, PlayerName},
        speed::Speed,
    },
    sprite_handles::Sprites,
};

pub fn spawn_opening_bundles(
    mut commands: Commands,
    sprite_handles: Res<Sprites>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    let texture_atlas = texture_atlases.get(&sprite_handles.atlas_handle).unwrap();
    let background_index = texture_atlas
        .get_texture_index(&sprite_handles.background_handle)
        .unwrap();
    let sprite_index = texture_atlas
        .get_texture_index(&sprite_handles.player_sprite_handle)
        .unwrap();
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.spawn_bundle(SpriteSheetBundle {
        transform: Transform {
            translation: Vec3::new(150.0, 0.0, 0.0),
            scale: Vec3::splat(4.0),
            ..Default::default()
        },
        sprite: TextureAtlasSprite::new(background_index as u32),
        texture_atlas: sprite_handles.atlas_handle.clone(),
        ..Default::default()
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
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::splat(2.0),
                ..Default::default()
            },
            ..Default::default()
        },
        player: Player,
    });

    state.set(AppState::Playing).unwrap(); // For now
}
