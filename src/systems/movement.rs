use bevy::{
    math::Vec3,
    prelude::{Commands, Mut, Query, QuerySet, Res, SpriteSheetBundle, Transform},
    render::camera::Camera,
    sprite::TextureAtlasSprite,
};

use crate::{
    components::{
        camera::GameCamera,
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
    mut query_set: QuerySet<(
        Query<(&Player, &Transform)>,
        Query<(&GameCamera, &Camera, &mut Transform)>,
    )>,
) {
    let mut player_x = 0.0;
    let mut player_y = 0.0;

    for e in query_set.q0_mut().iter_mut() {
        let (_, transform): (&Player, &Transform) = e;
        player_x = transform.translation.x;
        player_y = transform.translation.y;
    }

    for camera_data in query_set.q1_mut().iter_mut() {
        let (_, _, mut camera_transform): (&GameCamera, &Camera, Mut<'_, Transform>) = camera_data;

        camera_transform.translation.x = player_x;
        camera_transform.translation.y = player_y;
    }
}

pub fn check_floor_collision(
    mut commands: Commands,
    sprites: Res<Sprites>,
    player_query: Query<(&Player, &Transform)>,
    mut grid_query: Query<&mut Grid>,
) {
    let (_, transform): (&Player, &Transform) = player_query.single().unwrap();

    let mut grid: Mut<'_, Grid> = grid_query.single_mut().unwrap();

    let bounding_box = BoundingBox::square(transform.translation.x, transform.translation.y, 60.0);

    for cell in grid.cells.iter_mut() {
        if cell.intersects_box(&bounding_box) && cell.sprite.is_none() {
            let entity_commands = commands.spawn_bundle(SpriteSheetBundle {
                transform: Transform {
                    translation: cell.cell_center.clone(),
                    scale: Vec3::splat(2.0),
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
