mod app_state;
mod components;
mod sprite_handles;
mod systems;

use app_state::AppState;
use bevy::{input::system::exit_on_esc_system, prelude::*};
use sprite_handles::{LoadedTextures, Sprites};
use systems::{
    initial_spawns::spawn_opening_bundles,
    inputs::keyboard_input_system,
    player_movement::calculate_movement,
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
        .add_system_set(SystemSet::on_enter(AppState::TexturesLoaded).with_system(load_sprites.system()))
        .add_system_set(
            SystemSet::on_enter(AppState::SpritesLoaded).with_system(spawn_opening_bundles.system()),
        )
        .add_system_set(
            SystemSet::on_update(AppState::Playing)
                .with_system(keyboard_input_system.system().label("input"))
                .with_system(
                    calculate_movement
                        .system()
                        .label("player_movement")
                        .after("input"),
                ),
        )
        .add_system(exit_on_esc_system.system())
        .run();
}
