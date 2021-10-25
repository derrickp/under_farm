use bevy::{
    prelude::{Handle, HandleUntyped},
    sprite::TextureAtlas,
};

#[derive(Default)]
pub struct Sprites {
    pub atlas_handle: Handle<TextureAtlas>,
    pub player_sprite_index: usize,
    pub dirt_floor_indexes: Vec<usize>,
    pub outer_wall_index: usize,
    pub room_wall_index: usize,
    pub room_floor_index: usize,
    pub broken_wall_index: usize,
    pub table_index: usize,
    pub brick_wall_cracked_index: usize,
    pub brick_wall_really_cracked_index: usize,
    pub broken_small_table: usize,
}

#[derive(Default)]
pub struct LoadedTextures {
    pub handles: Vec<HandleUntyped>,
}
