use bevy::{
    math::Vec3,
    prelude::{Commands, Mut, Query, Res, SpriteSheetBundle, Transform},
    render::camera::Camera,
    sprite::TextureAtlasSprite,
};

use crate::{
    components::{
        grid::{BoundingBox, Grid},
        player::Player,
        speed::Speed,
    },
    sprites::Sprites,
};

pub fn player_movement(mut query: Query<(&Player, &Speed, &mut Transform)>) {
    let (_, speed, mut transform): (&Player, &Speed, Mut<'_, Transform>) =
        query.single_mut().unwrap();

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
    sprites: Res<Sprites>,
    player_query: Query<(&Player, &Transform)>,
    mut grid_query: Query<&mut Grid>,
) {
    let (_, transform): (&Player, &Transform) = player_query.single().unwrap();

    let mut grid: Mut<'_, Grid> = grid_query.single_mut().unwrap();

    let bounding_box = BoundingBox {
        min_x: transform.translation.x.floor(),
        max_x: transform.translation.x.floor(),
        min_y: transform.translation.y.floor(),
        max_y: transform.translation.y.floor(),
    };

    for cell in grid.cells.iter_mut() {
        if cell.intersects_box(&bounding_box) && cell.sprite.is_none() {
            let entity_commands = commands.spawn_bundle(SpriteSheetBundle {
                transform: Transform {
                    translation: cell.cell_center.clone(),
                    scale: Vec3::splat(1.0),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite::new(sprites.background_index as u32),
                texture_atlas: sprites.atlas_handle.clone(),
                ..Default::default()
            });
            cell.sprite = Some(entity_commands.id());
            // let outline_entity = cell.outline.unwrap();
            // commands
            //     .entity(outline_entity)
            //     .remove_bundle::<SpriteSheetBundle>();
            // cell.outline = None;
        }
    }
}
