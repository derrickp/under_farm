mod app_state;
mod components;
mod sprite_handles;
mod systems;

use app_state::AppState;
use bevy::prelude::*;
use sprite_handles::{LoadedTextures, Sprites};
use systems::{
    initial_spawns::spawn_opening_bundles,
    textures::{check_textures, load_sprites, load_textures},
};

fn main() {
    App::build()
        .init_resource::<Sprites>()
        .init_resource::<LoadedTextures>()
        .add_state(AppState::Setup)
        .add_plugins(DefaultPlugins)
        .add_system_set(SystemSet::on_enter(AppState::Setup).with_system(load_textures.system()))
        .add_system_set(SystemSet::on_update(AppState::Setup).with_system(check_textures.system()))
        .add_system_set(SystemSet::on_enter(AppState::Finished).with_system(load_sprites.system()))
        .add_system_set(
            SystemSet::on_enter(AppState::Playing).with_system(spawn_opening_bundles.system()),
        )
        .run();
}
