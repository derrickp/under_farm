use bevy::{
    asset::LoadState,
    prelude::{AssetServer, Assets, Res, ResMut, Texture},
    sprite::{TextureAtlas, TextureAtlasBuilder},
};

use crate::{
    configuration::{crops::CropConfigurations, sprites::dirt_floor_sprite_names},
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
    mut crop_configurations: ResMut<CropConfigurations>,
    loaded_textures: Res<LoadedTextures>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Texture>>,
    mut load_state: ResMut<GameLoadState>,
    asset_server: Res<AssetServer>,
) {
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in loaded_textures.handles.iter() {
        let texture = textures.get(handle).unwrap();
        texture_atlas_builder.add_texture(handle.clone_weak().typed::<Texture>(), texture);
    }

    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();

    for config in crop_configurations.configurations.as_mut_slice() {
        for mut stage in config.stages.as_mut_slice() {
            let handle = asset_server.get_handle(stage.sprite_location);
            if let Some(index) = texture_atlas.get_texture_index(&handle) {
                stage.sprite_index = Some(index as u32);
            }
        }
    }

    sprites.dirt_floor_indexes = Vec::new();
    for name in dirt_floor_sprite_names() {
        let handle = asset_server.get_handle(name);
        if let Some(index) = texture_atlas.get_texture_index(&handle) {
            sprites.dirt_floor_indexes.push(index);
        }
    }

    let texture_handle = asset_server.load("sprites/goblin_big_hat.png");
    let outline_handle = asset_server.get_handle("sprites/cell_outline_32.png");
    let outer_wall_handle = asset_server.get_handle("sprites/wall.png");
    let room_wall_handle = asset_server.get_handle("sprites/brick_wall.png");
    let room_floor_handle = asset_server.get_handle("sprites/sand_1.png");
    let broken_wall_handle = asset_server.get_handle("sprites/broken_wall.png");
    let table_handle = asset_server.get_handle("sprites/small_table.png");
    let brick_wall_cracked = asset_server.get_handle("sprites/brick_wall_cracked.png");
    let brick_wall_really_cracked =
        asset_server.get_handle("sprites/brick_wall_really_cracked.png");
    let broken_small_table = asset_server.get_handle("sprites/broken_small_table.png");

    sprites.player_sprite_index = texture_atlas.get_texture_index(&texture_handle).unwrap();
    sprites.outline_index = texture_atlas.get_texture_index(&outline_handle).unwrap();
    sprites.outer_wall_index = texture_atlas.get_texture_index(&outer_wall_handle).unwrap();
    sprites.room_wall_index = texture_atlas.get_texture_index(&room_wall_handle).unwrap();
    sprites.room_floor_index = texture_atlas.get_texture_index(&room_floor_handle).unwrap();
    sprites.broken_wall_index = texture_atlas
        .get_texture_index(&broken_wall_handle)
        .unwrap();
    sprites.table_index = texture_atlas.get_texture_index(&table_handle).unwrap();
    sprites.brick_wall_cracked_index = texture_atlas
        .get_texture_index(&brick_wall_cracked)
        .unwrap();
    sprites.brick_wall_really_cracked_index = texture_atlas
        .get_texture_index(&brick_wall_really_cracked)
        .unwrap();
    sprites.broken_small_table = texture_atlas
        .get_texture_index(&broken_small_table)
        .unwrap();

    let atlas_handle = texture_atlases.add(texture_atlas);
    sprites.atlas_handle = atlas_handle;

    load_state.textures_set = true;
}
