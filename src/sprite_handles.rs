use bevy::{
    prelude::{Handle, HandleUntyped, Texture},
    sprite::TextureAtlas,
};

#[derive(Default)]
pub struct Sprites {
    pub atlas_handle: Handle<TextureAtlas>,
    pub player_sprite_handle: Handle<Texture>,
    pub background_handle: Handle<Texture>,
    pub outline_handle: Handle<Texture>,
}

#[derive(Default)]
pub struct LoadedTextures {
    pub handles: Vec<HandleUntyped>,
}
