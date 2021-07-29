mod components;
mod sprites;
mod states;
mod systems;

use bevy::{input::system::exit_on_esc_system, prelude::*};
use sprites::{LoadedTextures, Sprites};
use states::{AppState, GameState};
use systems::{
    actions::crop_actions,
    initial_spawns::spawn_opening_bundles,
    inputs::{action_input_system, movement_input_system, MovementInputTimer},
    movement::{camera_movement, check_floor_collision, player_movement},
    textures::{check_textures, load_sprites, load_textures},
};

fn main() {
    App::build()
        .init_resource::<Sprites>()
        .init_resource::<LoadedTextures>()
        .init_resource::<GameState>()
        .insert_resource(MovementInputTimer(Timer::from_seconds(0.1, true)))
        .add_state(AppState::Setup)
        .add_plugins(DefaultPlugins)
        .add_system_set(SystemSet::on_enter(AppState::Setup).with_system(load_textures.system()))
        .add_system_set(SystemSet::on_update(AppState::Setup).with_system(check_textures.system()))
        .add_system_set(SystemSet::on_enter(AppState::Finished).with_system(load_sprites.system()))
        .add_system_set(
            SystemSet::on_enter(AppState::Playing)
                .with_system(spawn_opening_bundles.system().label("opening_spawn")),
        )
        .add_system_set(
            SystemSet::on_update(AppState::Playing)
                .with_system(movement_input_system.system().label("movement_input"))
                .with_system(action_input_system.system().label("action_input"))
                .with_system(
                    player_movement
                        .system()
                        .label("player_movement")
                        .after("movement_input"),
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
                )
                .with_system(
                    crop_actions
                        .system()
                        .label("crop_actions")
                        .after("player_movement"),
                ),
        )
        .add_system(exit_on_esc_system.system())
        .run();
}
