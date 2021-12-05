use bevy::prelude::{Commands, Entity, OrthographicCameraBundle, Query, UiCameraBundle};

use crate::components::cameras::{GameCamera, GameCameraState, UiCamera};

pub fn remove_gameplay_camera(mut commands: Commands, query: Query<(&GameCamera, Entity)>) {
    if query.is_empty() {
        return;
    }

    let (_, entity): (&GameCamera, Entity) = query.single();
    commands.entity(entity).despawn();
}

pub fn add_gameplay_camera(
    mut commands: Commands,
    query: Query<&GameCamera>,
    camera_state_query: Query<&GameCameraState>,
) {
    if !query.is_empty() {
        return;
    }

    let mut ortho_camera = OrthographicCameraBundle::new_2d();

    if !camera_state_query.is_empty() {
        let camera_state = camera_state_query.single();
        ortho_camera.transform.scale = camera_state.scale;
    }

    commands.spawn_bundle(ortho_camera).insert(GameCamera);
}

pub fn add_ui_camera(mut commands: Commands, query: Query<&UiCamera>) {
    if !query.is_empty() {
        return;
    }

    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(UiCamera);
}
