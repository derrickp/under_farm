use bevy::{
    prelude::{Handle, HandleUntyped},
    sprite::TextureAtlas,
};

#[derive(Default)]
pub struct Sprites {
    pub atlas_handle: Handle<TextureAtlas>,
    pub player_sprite_index: usize,
    pub background_index: usize,
    pub outline_index: usize,
}

#[derive(Default)]
pub struct LoadedTextures {
    pub handles: Vec<HandleUntyped>,
}
