mod components;
mod configuration;
mod plugins;
mod sprites;
mod states;
mod systems;

use bevy::{input::system::exit_on_esc_system, prelude::*};
use configuration::game::GameConfiguration;
use plugins::{inventory::InventoryPlugin, world::WorldPlugin};
use sprites::{LoadedTextures, Sprites};
use states::{AppState, GameLoadState};
use systems::{
    actions::{
        clear_structure_action, crop_actions, dig_action, hit_actions, pickup_actions,
        reset_hit_actions, reset_pickup_actions,
    },
    cameras::{add_gameplay_camera, add_ui_camera},
    crops::grow_crops_system,
    initial_spawns::{spawn_opening_bundles, spawn_player_text},
    inputs::{
        action_input_system, movement_input_system, reset_action_input_system,
        toggle_coordinates_system, zoom_camera_system, MovementInputTimer,
    },
    loading::{check_load_state, start_game},
    movement::{
        camera_movement, check_floor_collision, check_item_pickup, player_movement,
        update_player_grid_coordinate, update_player_text,
    },
    spawns::{
        drop_floor, reset_crop_spawns, reset_structure_spawns, spawn_crops, spawn_structures,
    },
    textures::{check_textures, load_sprites, load_textures},
    world::generate_world_grid,
};

fn main() {
    // TODO Should probably move this at some point...
    let game_config = GameConfiguration::load("./assets/config");

    App::new()
        .insert_resource(ClearColor(Color::rgb(0.05, 0.05, 0.05)))
        .init_resource::<Sprites>()
        .init_resource::<LoadedTextures>()
        .init_resource::<GameLoadState>()
        .insert_resource(game_config)
        .init_resource::<MovementInputTimer>()
        .add_state(AppState::Startup)
        .add_plugins(DefaultPlugins)
        .add_plugin(InventoryPlugin)
        .add_plugin(WorldPlugin)
        .add_system_set(SystemSet::on_enter(AppState::Startup).with_system(load_textures.system()))
        .add_system_set(
            SystemSet::on_update(AppState::Startup)
                .with_system(check_textures.system().label("check_textures"))
                .with_system(check_load_state.system().after("check_textures")),
        )
        .add_system_set(
            SystemSet::on_enter(AppState::FinishedLoading)
                .with_system(load_sprites.system())
                .with_system(generate_world_grid.system()),
        )
        .add_system_set(
            SystemSet::on_update(AppState::FinishedLoading).with_system(start_game.system()),
        )
        .add_system_set(
            SystemSet::on_enter(AppState::InGame)
                .with_system(spawn_opening_bundles.system().label("opening_spawn"))
                .with_system(spawn_player_text.system())
                .with_system(add_gameplay_camera.system())
                .with_system(add_ui_camera.system()),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(movement_input_system.system().label("movement_input"))
                .with_system(
                    player_movement
                        .system()
                        .label("player_movement")
                        .after("movement_input"),
                )
                .with_system(
                    action_input_system
                        .system()
                        .label("action_input")
                        .after("player_movement"),
                )
                .with_system(
                    dig_action
                        .system()
                        .label("dig_action")
                        .after("action_input"),
                )
                .with_system(
                    reset_action_input_system
                        .system()
                        .after("crop_actions")
                        .after("dig_action")
                        .after("clear_structure_action")
                        .after("drop_floor"),
                )
                .with_system(
                    check_item_pickup
                        .system()
                        .label("check_item_pickup")
                        .after("player_movement"),
                )
                .with_system(
                    update_player_grid_coordinate
                        .system()
                        .label("update_player_grid_coordinate")
                        .after("player_movement"),
                )
                .with_system(
                    update_player_text
                        .system()
                        .after("update_player_grid_coordinate"),
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
                    hit_actions
                        .system()
                        .label("hit_actions")
                        .after("player_movement"),
                )
                .with_system(
                    pickup_actions
                        .system()
                        .label("pickup_actions")
                        .after("check_item_pickup"),
                )
                .with_system(
                    clear_structure_action
                        .system()
                        .label("clear_structure_action")
                        .after("action_input"),
                )
                .with_system(reset_hit_actions.system().after("hit_actions"))
                .with_system(reset_pickup_actions.system().after("pickup_actions"))
                .with_system(
                    crop_actions
                        .system()
                        .label("crop_actions")
                        .after("action_input"),
                )
                .with_system(zoom_camera_system.system())
                .with_system(toggle_coordinates_system.system())
                .with_system(
                    grow_crops_system
                        .system()
                        .label("grow_crops_system")
                        .after("tick_game_world"),
                )
                .with_system(
                    spawn_crops
                        .system()
                        .label("spawn_crops")
                        .after("crop_actions")
                        .after("grow_crops_system"),
                )
                .with_system(
                    spawn_structures
                        .system()
                        .label("spawn_structures")
                        .after("dig_action"),
                )
                .with_system(reset_structure_spawns.system().after("spawn_structures"))
                .with_system(
                    drop_floor
                        .system()
                        .after("action_input")
                        .label("drop_floor"),
                )
                .with_system(reset_crop_spawns.system().after("spawn_crops")),
        )
        .add_system(exit_on_esc_system.system())
        .run();
}
