use bevy::{
    prelude::{Handle, HandleUntyped, Resource},
    sprite::TextureAtlas,
};

#[derive(Default, Resource)]
pub struct Sprites {
    pub atlas_handle: Handle<TextureAtlas>,
}

#[derive(Default, Resource)]
pub struct LoadedTextures {
    pub handles: Vec<HandleUntyped>,
}
