use bevy::{math::Vec3, prelude::Component};

#[derive(Component)]
pub struct GameCamera;

#[derive(Component)]
pub struct UiCamera;

#[derive(Component)]
pub struct GameCameraState {
    pub scale: Vec3,
}

impl Default for GameCameraState {
    fn default() -> Self {
        Self {
            scale: Vec3::splat(1.0),
        }
    }
}
