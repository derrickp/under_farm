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
    movement::{camera_movement, check_floor_collision, player_movement},
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
        .add_system_set(
            SystemSet::on_update(AppState::Playing)
                .with_system(keyboard_input_system.system().label("input"))
                .with_system(
                    player_movement
                        .system()
                        .label("player_movement")
                        .after("input"),
                )
                .with_system(
                    camera_movement
                        .system()
                        .label("camera_movement")
                        .after("player_movement"),
                )
                .with_system(
                    check_floor_collision
                        .system()
                        .label("floor_collisions")
                        .after("player_movement"),
                ),
        )
        .add_system(exit_on_esc_system.system())
        .run();
}
