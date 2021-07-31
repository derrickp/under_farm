mod components;
mod configuration;
mod sprites;
mod states;
mod systems;

use bevy::{input::system::exit_on_esc_system, prelude::*};
use configuration::crops::CropConfigurations;
use sprites::{LoadedTextures, Sprites};
use states::{AppState, GameState, InventoryState};
use systems::{
    actions::crop_actions,
    cameras::{add_gameplay_camera, remove_gameplay_camera},
    initial_spawns::spawn_opening_bundles,
    inputs::{
        action_input_system, movement_input_system, open_close_inventory_input_system,
        MovementInputTimer,
    },
    inventory::{add_inventory_text, remove_inventory_text},
    movement::{camera_movement, check_floor_collision, player_movement},
    textures::{check_textures, load_sprites, load_textures},
};

fn main() {
    App::build()
        .init_resource::<Sprites>()
        .init_resource::<LoadedTextures>()
        .init_resource::<GameState>()
        .init_resource::<InventoryState>()
        .init_resource::<CropConfigurations>()
        .insert_resource(MovementInputTimer(Timer::from_seconds(0.1, true)))
        .add_state(AppState::Startup)
        .add_plugins(DefaultPlugins)
        .add_system_set(SystemSet::on_enter(AppState::Startup).with_system(load_textures.system()))
        .add_system_set(
            SystemSet::on_update(AppState::Startup).with_system(check_textures.system()),
        )
        .add_system_set(
            SystemSet::on_enter(AppState::FinishedLoading).with_system(load_sprites.system()),
        )
        .add_system_set(
            SystemSet::on_enter(AppState::InGame)
                .with_system(spawn_opening_bundles.system().label("opening_spawn")),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
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
        .add_system_set(
            SystemSet::on_enter(AppState::InventoryScreen)
                .with_system(remove_gameplay_camera.system())
                .with_system(add_inventory_text.system()),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::InventoryScreen)
                .with_system(add_gameplay_camera.system())
                .with_system(remove_inventory_text.system()),
        )
        .add_system(open_close_inventory_input_system.system())
        .add_system(exit_on_esc_system.system())
        .run();
}
