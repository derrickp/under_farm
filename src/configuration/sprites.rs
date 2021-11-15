use bevy::math::Vec3;

pub const CROP_SPRITE_SCALE: f32 = 3.0;
pub const SPRITE_SCALE: f32 = 4.0;
pub const PLAYER_SPRITE_SCALE: f32 = 2.0;
pub const SPRITE_SIZE: f32 = 32.0;

pub fn sprite_scale() -> Vec3 {
    Vec3::splat(SPRITE_SCALE)
}

pub fn player_sprite_scale() -> Vec3 {
    Vec3::splat(PLAYER_SPRITE_SCALE)
}
