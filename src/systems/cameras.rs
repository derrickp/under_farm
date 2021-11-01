use bevy::prelude::{Commands, Entity, OrthographicCameraBundle, Query, Res, UiCameraBundle};

use crate::{
    components::cameras::{GameCamera, UiCamera},
    states::GameState,
};

pub fn remove_gameplay_camera(mut commands: Commands, query: Query<(&GameCamera, Entity)>) {
    if let Ok(data) = query.single() {
        let (_, entity): (&GameCamera, Entity) = data;
        commands.entity(entity).despawn();
    }
}

pub fn add_gameplay_camera(
    mut commands: Commands,
    game_state: Res<GameState>,
    query: Query<&GameCamera>,
) {
    if query.single().is_err() {
        let mut ortho_camera = OrthographicCameraBundle::new_2d();
        ortho_camera.transform.scale = game_state.game_camera_scale;
        commands.spawn_bundle(ortho_camera).insert(GameCamera);
    }
}

pub fn add_ui_camera(mut commands: Commands, query: Query<&UiCamera>) {
    if query.single().is_err() {
        commands
            .spawn_bundle(UiCameraBundle::default())
            .insert(UiCamera);
    }
}
