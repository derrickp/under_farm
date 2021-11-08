use bevy::math::Vec3;

pub const CROP_SPRITE_SCALE: f32 = 3.0;
pub const SPRITE_SCALE: f32 = 4.0;
pub const PLAYER_SPRITE_SCALE: f32 = 2.0;
pub const SPRITE_SIZE: f32 = 32.0;

pub const BRICK_WALL: &str = "sprites/brick_wall.png";
pub const BRICK_WALL_MINOR_CRACKED: &str = "sprites/brick_wall_minor_cracked.png";
pub const BRICK_WALL_CRACKED: &str = "sprites/brick_wall_cracked.png";
pub const BRICK_WALL_REALLY_CRACKED: &str = "sprites/brick_wall_really_cracked.png";
pub const BRICK_WALL_RUBBLE: &str = "sprites/broken_wall.png";

pub const GOBLIN_BIG_HAT: &str = "sprites/goblin_big_hat.png";

pub const SMALL_TABLE: &str = "sprites/small_table.png";
pub const BROKEN_SMALL_TABLE: &str = "sprites/broken_small_table.png";

pub const DIRT_FLOOR_1: &str = "sprites/purple_floor_1.png";
pub const DIRT_FLOOR_2: &str = "sprites/purple_floor_2.png";
pub const DIRT_FLOOR_3: &str = "sprites/purple_floor_3.png";
pub const DIRT_FLOOR_4: &str = "sprites/purple_floor_4.png";
pub const DIRT_FLOOR_5: &str = "sprites/purple_floor_5.png";

pub const ROOM_FLOOR_1: &str = "sprites/sand_1.png";

pub const UNBREAKABLE_WALL: &str = "sprites/wall.png";

pub const GIANT_MUSHROOM: &str = "sprites/giant_mushroom.png";
pub const GIANT_MUSHROOM_SPORES: &str = "sprites/giant_mushroom_spores.png";
pub const KANE_SEEDS: &str = "sprites/kane_seeds.png";
pub const KANE_STALKS: &str = "sprites/kane_stalks.png";
pub const POTATO_SEEDS: &str = "sprites/potato_seeds.png";
pub const POTATOES: &str = "sprites/potatoes.png";
pub const TURNIP_SEED: &str = "sprites/turnip_seed.png";
pub const TURNIP_TOP: &str = "sprites/turnip_top.png";

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
