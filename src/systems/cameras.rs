use bevy::prelude::{Commands, Entity, OrthographicCameraBundle, Query, UiCameraBundle};

use crate::components::cameras::{GameCamera, GameCameraState, UiCamera};

pub fn remove_gameplay_camera(mut commands: Commands, query: Query<(&GameCamera, Entity)>) {
    let data = match query.single() {
        Ok(it) => it,
        _ => return,
    };

    let (_, entity): (&GameCamera, Entity) = data;
    commands.entity(entity).despawn();
}

pub fn add_gameplay_camera(
    mut commands: Commands,
    query: Query<&GameCamera>,
    camera_state_query: Query<&GameCameraState>,
) {
    if query.single().is_ok() {
        return;
    }

    let mut ortho_camera = OrthographicCameraBundle::new_2d();

    if let Ok(camera_state) = camera_state_query.single() {
        ortho_camera.transform.scale = camera_state.scale;
    }

    commands.spawn_bundle(ortho_camera).insert(GameCamera);
}

pub fn add_ui_camera(mut commands: Commands, query: Query<&UiCamera>) {
    if query.single().is_ok() {
        return;
    }

    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(UiCamera);
}
