use bevy::{
    math::Vec3,
    prelude::{Commands, Entity, Mut, Query, Res, ResMut, Transform},
};
use tdlg::cells::layer::LayerType;

use crate::{
    components::{
        action::{CurrentAction, InteractAction},
        body::Body,
        crop::CropBundle,
        ground::GroundTileBundle,
        item::{ItemBundle, ItemType},
        player::{Player, PlayerBundle},
        spawns::{MapSpawn, Spawns},
        structure::StructureBundle,
        tool::ToolType,
    },
    configuration::{game::GameConfiguration, map::world_coordinate_from_grid},
    sprites::Sprites,
};

pub fn spawn_crops(
    mut commands: Commands,
    sprites: Res<Sprites>,
    game_config: Res<GameConfiguration>,
    query: Query<&Spawns>,
) {
    if query.is_empty() {
        return;
    }

    let spawns: &Spawns = query.single();

    if spawns.crops.is_empty() {
        return;
    }

    for spawn in spawns.crops.iter() {
        commands.spawn_bundle(CropBundle::build(
            spawn,
            &sprites,
            &spawn.config,
            game_config.sprite_config.crop_scale,
            game_config.sprite_config.scale,
        ));
    }
}

pub fn spawn_structures(
    mut commands: Commands,
    sprites: Res<Sprites>,
    game_config: Res<GameConfiguration>,
    query: Query<&Spawns>,
) {
    if query.is_empty() {
        return;
    }

    let spawns: &Spawns = query.single();

    if spawns.structures.is_empty() {
        return;
    }

    for spawn in spawns.structures.iter() {
        let structure_config = match game_config
            .structures_config
            .config_by_key(spawn.structure_key)
        {
            Some(it) => it,
            _ => continue,
        };

        commands.spawn_bundle(StructureBundle::build(
            spawn.position,
            &sprites.atlas_handle,
            structure_config,
            &game_config.sprite_config,
            game_config.tile_size(),
        ));
    }
}

pub fn reset_crop_spawns(mut query: Query<&mut Spawns>) {
    if query.is_empty() {
        return;
    }

    let mut spawns: Mut<Spawns> = query.single_mut();

    spawns.crops.clear();
}

pub fn reset_structure_spawns(mut query: Query<&mut Spawns>) {
    if query.is_empty() {
        return;
    }

    let mut spawns: Mut<Spawns> = query.single_mut();

    spawns.structures.clear();
}

pub fn drop_floor(
    mut commands: Commands,
    mut player_query: Query<(&Player, &CurrentAction)>,
    structure_query: Query<(&Body, Entity)>,
    mut game_config: ResMut<GameConfiguration>,
    mut spawns_query: Query<&mut Spawns>,
) {
    if player_query.is_empty() {
        return;
    }

    if spawns_query.is_empty() {
        return;
    }

    let (_, action): (&Player, &CurrentAction) = player_query.single_mut();

    if let Some(InteractAction::DropFloors) = action.interact {
        let world = game_config.generator(true).generate_top_down_map().unwrap();

        for structure_data in structure_query.iter() {
            let (_, entity): (&Body, Entity) = structure_data;

            commands.entity(entity).despawn();
        }

        let mut spawns: Mut<Spawns> = spawns_query.single_mut();
        spawns.map_spawn = Some(MapSpawn { map: world });
    }
}

pub fn reset_spawn_map(mut query: Query<&mut Spawns>) {
    if query.is_empty() {
        return;
    }

    let mut spawns: Mut<Spawns> = query.single_mut();
    spawns.map_spawn = None;
}

pub fn spawn_map(
    mut commands: Commands,
    query: Query<&Spawns>,
    mut player_query: Query<(&Player, &mut Transform)>,
    sprites: Res<Sprites>,
    game_config: Res<GameConfiguration>,
) {
    if query.is_empty() {
        return;
    }

    let spawns: &Spawns = query.single();

    let map_spawn = match &spawns.map_spawn {
        Some(it) => it,
        _ => return,
    };

    let player_spawn = map_spawn.map.entry_coordinate;
    let coordinate = world_coordinate_from_grid(
        &player_spawn,
        game_config.world_config.world_stats.map_size,
        game_config.tile_size(),
    );
    if player_query.is_empty() {
        let player_bundle = PlayerBundle::build_main_player(coordinate, &sprites, &game_config);
        commands.spawn_bundle(player_bundle);
    } else {
        let (_, mut transform): (&Player, Mut<Transform>) = player_query.single_mut();
        transform.translation.x = coordinate.x;
        transform.translation.y = coordinate.y;
    }

    let mut spawned_shovel = false;
    for cell in map_spawn.map.grid.cells.values() {
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
                    if !underground {
                        if spawned_shovel {
                            println!("spawned hoe {:?}", &cell.coordinate);
                            if let Some(tool) = game_config.tool_configs.tool_by_type(ToolType::Hoe)
                            {
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
                        } else if let Some(tool) =
                            game_config.tool_configs.tool_by_type(ToolType::Shovel)
                        {
                            println!("spawned shovel {:?}", &cell.coordinate);
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
                            spawned_shovel = true;
                        }
                    }
                }
                _ => {}
            }
        }
    }

    let exit_coordinate = world_coordinate_from_grid(
        &map_spawn.map.exit_coordinate,
        game_config.world_config.world_stats.map_size,
        game_config.tile_size(),
    );

    println!(
        "{} {}",
        &map_spawn.map.exit_coordinate.x, &map_spawn.map.exit_coordinate.y
    );

    let structure_config = game_config.structures_config.config_by_key("exit").unwrap();
    let position = Vec3::new(exit_coordinate.x, exit_coordinate.y, 2.0);
    commands.spawn_bundle(StructureBundle::build(
        position,
        &sprites.atlas_handle,
        structure_config,
        &game_config.sprite_config,
        game_config.tile_size(),
    ));
}
