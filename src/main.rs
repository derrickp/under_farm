mod components;
mod configuration;
mod sprites;
mod states;
mod systems;

use bevy::prelude::*;
use configuration::game::GameConfiguration;
use sprites::{LoadedTextures, Sprites};
use states::{AppState, GameLoadState};
use systems::{
    actions::{
        clear_structure_action, crop_actions, dig_action, hit_actions, pickup_actions,
        reset_hit_actions, reset_pickup_actions,
    },
    cameras::add_gameplay_camera,
    crops::grow_crops_system,
    initial_spawns::{spawn_opening_bundles, spawn_player_text},
    inputs::{
        action_input_system, movement_input_system, reset_action_input_system,
        toggle_coordinates_system, zoom_camera_system, MovementInputTimer,
    },
    inventory::{
        add_current_selection, add_text, hide_game_sprites, open_close_inventory_input_system,
        remove_gameplay_camera, remove_text, remove_ui_camera, reset_selection, select_item,
        selection_input, show_game_sprites, update_text_colour,
    },
    loading::{check_load_state, start_game},
    movement::{
        camera_movement, check_floor_collision, check_item_pickup, player_movement,
        update_player_grid_coordinate, update_player_text,
    },
    spawns::{
        drop_floor, reset_crop_spawns, reset_spawn_map, reset_structure_spawns, spawn_crops,
        spawn_map, spawn_structures,
    },
    textures::{check_textures, load_sprites, load_textures},
    world::{check_world_actions, tick_game_world},
};

// System labels to enforce a run order of our systems
#[derive(SystemLabel, Debug, Clone, PartialEq, Eq, Hash)]
enum Label {
    CheckTextures,
    OpeningSpawn,
    MovementInput,
    PlayerMovement,
    ActionInput,
    DigAction,
    DropFloor,
    ClearStructureAction,
    CameraMovement,
    UpdatePlayerGridCoordinate,
    FloorCollisions,
    CropActions,
    CheckItemPickup,
    HitActions,
    PickupActions,
    GrowCrops,
    TickGameWorld,
    SpawnCrops,
    SpawnStructures,
    SpawnMap,
    ResetSpawnMap,
    ResetInventorySelection,
    SelectItem,
    InventoryInput,
    CheckWorldActions,
}

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
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_system_set(SystemSet::on_enter(AppState::Startup).with_system(load_textures))
        .add_system_set(
            SystemSet::on_update(AppState::Startup)
                .with_system(check_textures.label(Label::CheckTextures))
                .with_system(check_load_state.after(Label::CheckTextures)),
        )
        .add_system_set(SystemSet::on_enter(AppState::FinishedLoading).with_system(load_sprites))
        .add_system_set(SystemSet::on_update(AppState::FinishedLoading).with_system(start_game))
        .add_system_set(
            SystemSet::on_enter(AppState::InGame)
                .with_system(spawn_opening_bundles.label(Label::OpeningSpawn))
                .with_system(spawn_player_text)
                .with_system(add_gameplay_camera),
        )
        .add_system(open_close_inventory_input_system)
        .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(add_current_selection))
        .add_system_set(
            SystemSet::on_exit(AppState::FinishedLoading).with_system(systems::world::spawn),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(tick_game_world.label(Label::TickGameWorld))
                .with_system(
                    check_world_actions
                        .label(Label::CheckWorldActions)
                        .after(Label::TickGameWorld),
                ),
        )
        .add_system_set(
            SystemSet::on_enter(AppState::InventoryScreen)
                .with_system(add_text)
                .with_system(remove_gameplay_camera)
                .with_system(hide_game_sprites),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::InventoryScreen)
                .with_system(remove_text)
                .with_system(remove_ui_camera)
                .with_system(show_game_sprites),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InventoryScreen)
                .with_system(selection_input.label(Label::InventoryInput))
                .with_system(
                    select_item
                        .after(Label::InventoryInput)
                        .label(Label::SelectItem),
                )
                .with_system(update_text_colour.after(Label::InventoryInput))
                .with_system(
                    reset_selection
                        .label(Label::ResetInventorySelection)
                        .after(Label::SelectItem),
                ),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(movement_input_system.label(Label::MovementInput))
                .with_system(
                    player_movement
                        .label(Label::PlayerMovement)
                        .after(Label::MovementInput),
                )
                .with_system(
                    action_input_system
                        .label(Label::ActionInput)
                        .after(Label::PlayerMovement),
                )
                .with_system(dig_action.label(Label::DigAction).after(Label::ActionInput))
                .with_system(
                    reset_action_input_system
                        .after(Label::CropActions)
                        .after(Label::DigAction)
                        .after(Label::ClearStructureAction)
                        .after(Label::DropFloor),
                )
                .with_system(
                    check_item_pickup
                        .label(Label::CheckItemPickup)
                        .after(Label::PlayerMovement),
                )
                .with_system(
                    update_player_grid_coordinate
                        .label(Label::UpdatePlayerGridCoordinate)
                        .after(Label::PlayerMovement),
                )
                .with_system(update_player_text.after(Label::UpdatePlayerGridCoordinate))
                .with_system(
                    camera_movement
                        .label(Label::CameraMovement)
                        .after(Label::PlayerMovement),
                )
                .with_system(
                    check_floor_collision
                        .label(Label::FloorCollisions)
                        .after(Label::PlayerMovement),
                )
                .with_system(
                    hit_actions
                        .label(Label::HitActions)
                        .after(Label::PlayerMovement),
                )
                .with_system(
                    pickup_actions
                        .label(Label::PickupActions)
                        .after(Label::CheckItemPickup),
                )
                .with_system(
                    clear_structure_action
                        .label(Label::ClearStructureAction)
                        .after(Label::ActionInput),
                )
                .with_system(reset_hit_actions.after(Label::HitActions))
                .with_system(reset_pickup_actions.after(Label::PickupActions))
                .with_system(
                    crop_actions
                        .label(Label::CropActions)
                        .after(Label::ActionInput),
                )
                .with_system(zoom_camera_system)
                .with_system(toggle_coordinates_system)
                .with_system(
                    grow_crops_system
                        .label(Label::GrowCrops)
                        .after(Label::TickGameWorld),
                )
                .with_system(
                    spawn_crops
                        .label(Label::SpawnCrops)
                        .after(Label::CropActions)
                        .after(Label::GrowCrops),
                )
                .with_system(
                    spawn_structures
                        .label(Label::SpawnStructures)
                        .after(Label::DigAction),
                )
                .with_system(reset_structure_spawns.after(Label::SpawnStructures))
                .with_system(drop_floor.after(Label::ActionInput).label(Label::DropFloor))
                .with_system(reset_crop_spawns.after(Label::SpawnCrops))
                .with_system(spawn_map.label(Label::SpawnMap))
                .with_system(
                    reset_spawn_map
                        .label(Label::ResetSpawnMap)
                        .after(Label::SpawnMap),
                ),
        )
        .run();
}
