use bevy::{math::{Vec2, Vec3}, prelude::{Assets, Commands, Mut, Query, Res, SpriteSheetBundle, Transform}, render::camera::Camera, sprite::{collide_aabb::collide, TextureAtlas, TextureAtlasSprite}};

use crate::{
    components::{
        player::Player,
        speed::Speed,
        tile::{Tile, TileBundle},
    },
    sprite_handles::Sprites,
};

pub fn player_movement(mut query: Query<(&Player, &Speed, &mut Transform)>) {
    let (_, speed, mut transform): (&Player, &Speed, Mut<'_, Transform>) = query.single_mut().unwrap();

    let x = speed.current.x + transform.translation.x;
    let y = speed.current.y + transform.translation.y;

    transform.translation.x = x;
    transform.translation.y = y;
}

pub fn camera_movement(
    mut speed_query: Query<(&Player, &Speed)>,
    mut query: Query<(&Camera, &mut Transform)>,
) {
    let (_, speed): (&Player, &Speed) = speed_query.single_mut().unwrap();
    let (_, mut camera_transform): (&Camera, Mut<'_, Transform>) = query.single_mut().unwrap();

    camera_transform.translation.x = speed.current.x + camera_transform.translation.x;
    camera_transform.translation.y = speed.current.y + camera_transform.translation.y;
}

pub fn check_floor_collision(
    mut commands: Commands,
    sprite_handles: Res<Sprites>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    player_query: Query<(&Player, &Transform)>,
    mut floor_query: Query<(&Tile, &Transform)>,
) {
    let (_, transform): (&Player, &Transform) = player_query.single().unwrap();

    let mut collided: bool = false;

    for data in floor_query.iter_mut() {
        let (_, tile_transform): (&Tile, &Transform) = data;
        let collision = collide(
            transform.translation,
            Vec2::new(24.0, 24.0),
            tile_transform.translation,
            Vec2::new(24.0, 24.0),
        );

        if let Some(_) = collision {
            collided = true;
            break;
        }
    }

    if !collided {
        let texture_atlas = texture_atlases.get(&sprite_handles.atlas_handle).unwrap();
        let background_index = texture_atlas
            .get_texture_index(&sprite_handles.background_handle)
            .unwrap();
        commands.spawn_bundle(TileBundle {
            tile: Tile,
            sprite: SpriteSheetBundle {
                transform: Transform {
                    translation: Vec3::new(transform.translation.x, transform.translation.y, 0.0),
                    scale: Vec3::splat(2.0),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite::new(background_index as u32),
                texture_atlas: sprite_handles.atlas_handle.clone(),
                ..Default::default()
            },
        });
    }
}
