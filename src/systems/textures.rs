use bevy::{
    asset::LoadState,
    prelude::{AssetServer, Assets, Res, ResMut, Texture},
    sprite::{TextureAtlas, TextureAtlasBuilder},
};

use crate::{
    configuration::game::GameConfiguration,
    sprites::{LoadedTextures, Sprites},
    states::GameLoadState,
};

pub fn load_textures(mut loaded_textures: ResMut<LoadedTextures>, asset_server: Res<AssetServer>) {
    loaded_textures.handles = asset_server.load_folder("sprites").unwrap();
}

pub fn check_textures(
    mut load_state: ResMut<GameLoadState>,
    sprite_handles: Res<LoadedTextures>,
    asset_server: Res<AssetServer>,
) {
    if let LoadState::Loaded =
        asset_server.get_group_load_state(sprite_handles.handles.iter().map(|handle| handle.id))
    {
        load_state.texture_load_complete = true;
    }
}

pub fn load_sprites(
    mut sprites: ResMut<Sprites>,
    loaded_textures: Res<LoadedTextures>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Texture>>,
    mut load_state: ResMut<GameLoadState>,
    asset_server: Res<AssetServer>,
    mut game_config: ResMut<GameConfiguration>,
) {
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in loaded_textures.handles.iter() {
        let texture = textures.get(handle).unwrap();
        texture_atlas_builder.add_texture(handle.clone_weak().typed::<Texture>(), texture);
    }

    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();

    for config in game_config.crops_config.configurations.as_mut_slice() {
        for mut stage in config.stages.as_mut_slice() {
            let handle = asset_server.get_handle(stage.sprite_location());
            if let Some(index) = texture_atlas.get_texture_index(&handle) {
                stage.sprite_index = Some(index as u32);
            }
        }
    }

    for config in game_config.structures_config.configurations.as_mut_slice() {
        for mut structure_health in config.health_configs.as_mut_slice() {
            let handle = asset_server.get_handle(structure_health.sprite_location());
            if let Some(index) = texture_atlas.get_texture_index(&handle) {
                structure_health.sprite_index = Some(index as u32);
            }
        }
    }

    for config in game_config.floors_config.configurations.as_mut_slice() {
        for mut sprite_options in config.sprite_options.as_mut_slice() {
            let handle = asset_server.get_handle(sprite_options.sprite_location());
            if let Some(index) = texture_atlas.get_texture_index(&handle) {
                sprite_options.sprite_index = Some(index as u32);
            }
        }
    }

    for config in game_config
        .player_config
        .sprite_configs
        .options
        .as_mut_slice()
    {
        let handle = asset_server.get_handle(config.sprite_location());
        if let Some(index) = texture_atlas.get_texture_index(&handle) {
            config.sprite_index = Some(index as u32);
        }
    }

    let atlas_handle = texture_atlases.add(texture_atlas);
    sprites.atlas_handle = atlas_handle;

    load_state.textures_set = true;
}
