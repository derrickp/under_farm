use bevy::{
    math::Vec3,
    prelude::{AssetServer, Commands, Query, Res},
    window::Windows,
};

use tdlg::{cells::layer::LayerType, coordinate::Coordinate, map::TopDownMap};

use crate::{
    components::{
        action::WorldActions,
        cameras::GameCameraState,
        ground::GroundTileBundle,
        item::{ItemBundle, ItemType},
        player::{Player, PlayerBundle},
        spawns::Spawns,
        structure::StructureBundle,
        text::{PlayerStatsText, PlayerStatsTextBundle},
        tool::ToolType,
    },
    configuration::{game::GameConfiguration, map::world_coordinate_from_grid},
    sprites::Sprites,
};

pub fn spawn_player_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Res<Windows>,
    query: Query<&PlayerStatsText>,
) {
    if !query.is_empty() {
        return;
    }

    let coordinate = Coordinate::splat(0);
    let player_text_bundle = PlayerStatsTextBundle::build(&coordinate, &asset_server, &windows);
    commands.spawn_bundle(player_text_bundle);
}

pub fn spawn_opening_bundles(
    mut commands: Commands,
    sprites: Res<Sprites>,
    map: Res<TopDownMap>,
    query: Query<&Player>,
    game_config: Res<GameConfiguration>,
) {
    if !query.is_empty() {
        return;
    }

    for cell in map.grid.cells.values() {
        for (index, layer) in cell.layers.iter().enumerate() {
            let coordinate = world_coordinate_from_grid(
                &cell.coordinate,
                game_config.map_size(),
                game_config.tile_size(),
            );
            let position = Vec3::new(coordinate.x, coordinate.y, index as f32);
            match *layer {
                LayerType::Floor => {
                    let floor_config = game_config
                        .floors_config
                        .config_by_key("cave_floor")
                        .unwrap();
                    commands.spawn_bundle(GroundTileBundle::build(
                        position,
                        &sprites,
                        floor_config,
                        game_config.sprite_config.scale,
                        game_config.tile_size(),
                    ));
                }
                LayerType::RoomWall => {
                    let structure_config = game_config
                        .structures_config
                        .config_by_key("room_wall")
                        .unwrap();
                    commands.spawn_bundle(StructureBundle::build(
                        position,
                        &sprites.atlas_handle,
                        structure_config,
                        &game_config.sprite_config,
                        game_config.tile_size(),
                    ));
                }
                LayerType::RoomFloor => {
                    let config = game_config
                        .floors_config
                        .config_by_key("room_floor")
                        .unwrap();
                    commands.spawn_bundle(GroundTileBundle::build(
                        position,
                        &sprites,
                        config,
                        game_config.sprite_config.scale,
                        game_config.tile_size(),
                    ));
                }
                LayerType::Door => {
                    let config = game_config
                        .floors_config
                        .config_by_key("room_floor")
                        .unwrap();
                    commands.spawn_bundle(GroundTileBundle::build(
                        position,
                        &sprites,
                        config,
                        game_config.sprite_config.scale,
                        game_config.tile_size(),
                    ));
                }
                LayerType::OuterWall => {
                    let structure_config = game_config
                        .structures_config
                        .config_by_key("outer_wall")
                        .unwrap();
                    commands.spawn_bundle(StructureBundle::build(
                        position,
                        &sprites.atlas_handle,
                        structure_config,
                        &game_config.sprite_config,
                        game_config.tile_size(),
                    ));
                }
                LayerType::Rubble => {
                    let structure_config = game_config
                        .structures_config
                        .config_by_key("rubble")
                        .unwrap();
                    commands.spawn_bundle(StructureBundle::build(
                        position,
                        &sprites.atlas_handle,
                        structure_config,
                        &game_config.sprite_config,
                        game_config.tile_size(),
                    ));
                }
                LayerType::Table => {
                    let structure_config = game_config
                        .structures_config
                        .config_by_key("table")
                        .unwrap();
                    commands.spawn_bundle(StructureBundle::build(
                        position,
                        &sprites.atlas_handle,
                        structure_config,
                        &game_config.sprite_config,
                        game_config.tile_size(),
                    ));
                }
                LayerType::Note => {
                    println!(
                        "Note {} {} {}",
                        index, &cell.coordinate.x, &cell.coordinate.y
                    );
                }
                LayerType::CommonItem => {
                    println!(
                        "common item {} {} {}",
                        index, &cell.coordinate.x, &cell.coordinate.y
                    );
                    let underground = cell.is_layer_underground(layer).unwrap_or(false);
                    if let Some(tool) = game_config.tool_configs.tool_by_type(ToolType::Shovel) {
                        let tool_bundle = ItemBundle::build(
                            position,
                            &sprites,
                            tool.sprite_index.unwrap(),
                            game_config.sprite_config.scale,
                            game_config.tile_size(),
                            underground,
                            ItemType::Tool(tool),
                        );
                        commands.spawn_bundle(tool_bundle);
                    }
                }
                _ => {}
            }
        }
    }

    let player_spawn = map.entry_coordinate;
    let coordinate = world_coordinate_from_grid(
        &player_spawn,
        game_config.world_config.world_stats.map_size,
        game_config.tile_size(),
    );
    let player_bundle = PlayerBundle::build_main_player(coordinate, &sprites, &game_config);
    commands.spawn_bundle(player_bundle);

    commands.spawn().insert(GameCameraState::default());
    commands.spawn().insert(Spawns::default());
    commands.spawn().insert(WorldActions::default());
}
