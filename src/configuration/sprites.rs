use bevy::math::Vec3;

pub const SPRITE_SCALE: f32 = 2.0;
pub const SPRITE_SIZE: f32 = 32.0;

pub fn sprite_scale() -> Vec3 {
    return Vec3::splat(SPRITE_SCALE);
}
