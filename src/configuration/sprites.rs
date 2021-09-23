use bevy::math::Vec3;

pub const SPRITE_SCALE: f32 = 4.0;
pub const SPRITE_SIZE: f32 = 32.0;

pub fn sprite_scale() -> Vec3 {
    Vec3::splat(SPRITE_SCALE)
}

pub fn dirt_floor_sprite_names() -> Vec<&'static str> {
    return vec![
        "sprites/dirt_floor_1.png",
        "sprites/dirt_floor_2.png",
        "sprites/dirt_floor_3.png",
    ];
}
