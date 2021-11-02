mod components;
mod configuration;
mod sprites;
mod states;
mod systems;
mod world;

use bevy::{input::system::exit_on_esc_system, prelude::*};
use configuration::{crops::CropConfigurations, tools::ToolConfigurations};
use sprites::{LoadedTextures, Sprites};
use states::{AppState, GameLoadState};
use systems::{
    actions::{crop_actions, hit_actions, reset_hit_actions},
    cameras::{add_gameplay_camera, add_ui_camera, remove_gameplay_camera},
    crops::grow_crops_system,
    initial_spawns::spawn_opening_bundles,
    inputs::{
        action_input_system, movement_input_system, open_close_inventory_input_system,
        reset_action_input_system, zoom_camera_system, MovementInputTimer,
    },
    inventory::{
        add_inventory_text, remove_inventory_text, select_crop, update_inventory_text_colour,
    },
    loading::{check_load_state, start_game},
    movement::{camera_movement, check_floor_collision, player_movement},
    spawns::{reset_crop_spawns, spawn_crops},
    textures::{check_textures, load_sprites, load_textures},
    world::{generate_world_grid, tick_game_world},
};
use world::WorldTickTimer;

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.05, 0.05, 0.05)))
        .init_resource::<Sprites>()
        .init_resource::<LoadedTextures>()
        .init_resource::<GameLoadState>()
        .init_resource::<CropConfigurations>()
        .init_resource::<ToolConfigurations>()
        .init_resource::<MovementInputTimer>()
        .init_resource::<WorldTickTimer>()
        .add_state(AppState::Startup)
        .add_plugins(DefaultPlugins)
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
                .with_system(add_gameplay_camera.system())
                .with_system(add_ui_camera.system()),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(movement_input_system.system().label("movement_input"))
                .with_system(action_input_system.system().label("action_input"))
                .with_system(reset_action_input_system.system().after("crop_actions"))
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
                    hit_actions
                        .system()
                        .label("hit_actions")
                        .after("player_movement"),
                )
                .with_system(reset_hit_actions.system().after("hit_actions"))
                .with_system(
                    crop_actions
                        .system()
                        .label("crop_actions")
                        .after("player_movement"),
                )
                .with_system(zoom_camera_system.system())
                .with_system(tick_game_world.system().label("tick_game_world"))
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
                .with_system(reset_crop_spawns.system().after("spawn_crops")),
        )
        .add_system_set(
            SystemSet::on_enter(AppState::InventoryScreen)
                .with_system(remove_gameplay_camera.system())
                .with_system(add_inventory_text.system()),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::InventoryScreen)
                .with_system(remove_inventory_text.system()),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InventoryScreen)
                .with_system(select_crop.system())
                .with_system(update_inventory_text_colour.system()),
        )
        .add_system(open_close_inventory_input_system.system())
        .add_system(exit_on_esc_system.system())
        .run();
}
