use bevy::{
    prelude::{Handle, HandleUntyped},
    sprite::TextureAtlas,
};

#[derive(Default)]
pub struct Sprites {
    pub atlas_handle: Handle<TextureAtlas>,
}

#[derive(Default)]
pub struct LoadedTextures {
    pub handles: Vec<HandleUntyped>,
}
