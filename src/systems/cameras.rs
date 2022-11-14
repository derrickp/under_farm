use bevy::prelude::{Camera2dBundle, Commands, Query};

use crate::components::cameras::{GameCamera, GameCameraState};

pub fn add_gameplay_camera(
    mut commands: Commands,
    query: Query<&GameCamera>,
    camera_state_query: Query<&GameCameraState>,
) {
    if !query.is_empty() {
        return;
    }

    let mut ortho_camera = Camera2dBundle::default();

    if !camera_state_query.is_empty() {
        let camera_state = camera_state_query.single();
        ortho_camera.transform.scale = camera_state.scale;
    } else {
        commands.spawn(GameCameraState::default());
    }

    commands.spawn(ortho_camera).insert(GameCamera);
}
