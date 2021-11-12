use bevy::math::Vec3;

pub const CROP_SPRITE_SCALE: f32 = 3.0;
pub const SPRITE_SCALE: f32 = 4.0;
pub const PLAYER_SPRITE_SCALE: f32 = 2.0;
pub const SPRITE_SIZE: f32 = 32.0;

pub const GOBLIN_BIG_HAT: &str = "sprites/goblin_big_hat.png";

pub const DIRT_FLOOR_1: &str = "sprites/purple_floor_1.png";
pub const DIRT_FLOOR_2: &str = "sprites/purple_floor_2.png";
pub const DIRT_FLOOR_3: &str = "sprites/purple_floor_3.png";
pub const DIRT_FLOOR_4: &str = "sprites/purple_floor_4.png";
pub const DIRT_FLOOR_5: &str = "sprites/purple_floor_5.png";

pub const ROOM_FLOOR_1: &str = "sprites/sand_1.png";

pub fn sprite_scale() -> Vec3 {
    Vec3::splat(SPRITE_SCALE)
}

pub fn player_sprite_scale() -> Vec3 {
    Vec3::splat(PLAYER_SPRITE_SCALE)
}

pub fn dirt_floor_sprite_names() -> Vec<&'static str> {
    return vec![
        DIRT_FLOOR_1,
        DIRT_FLOOR_2,
        DIRT_FLOOR_3,
        DIRT_FLOOR_4,
        DIRT_FLOOR_5,
    ];
}
