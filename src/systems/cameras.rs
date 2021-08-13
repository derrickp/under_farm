use bevy::prelude::{Commands, OrthographicCameraBundle, ResMut};

use crate::{components::camera::GameCamera, states::GameState};

pub fn remove_gameplay_camera(mut commands: Commands, mut game_state: ResMut<GameState>) {
    if let Some(camera_entity) = game_state.game_camera {
        commands.entity(camera_entity).despawn();
        game_state.game_camera = None;
    }
}

pub fn add_gameplay_camera(mut commands: Commands, mut game_state: ResMut<GameState>) {
    if let None = game_state.game_camera {
        let mut ortho_camera = OrthographicCameraBundle::new_2d();
        ortho_camera.transform.scale = game_state.game_camera_scale;
        let camera = commands.spawn_bundle(ortho_camera).insert(GameCamera).id();
        game_state.game_camera = Some(camera);
    }
}
