use bevy::math::Vec3;

pub struct GameCamera;

pub struct UiCamera;

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
