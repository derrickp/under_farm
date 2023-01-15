use bevy::{
    prelude::{AssetServer, Commands, Query, Res, ResMut},
    window::Windows,
};
use tdlg::map::cells::Coordinate;

use crate::{
    components::{
        player::Player,
        spawns::{MapSpawn, Spawns},
        text::{PlayerStatsText, PlayerStatsTextBundle},
    },
    configuration::game::GameConfiguration,
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

    let coordinate = Coordinate::from(0);
    let player_text_bundle = PlayerStatsTextBundle::build(&coordinate, &asset_server, &windows);
    commands.spawn(player_text_bundle);
}

pub fn spawn_opening_bundles(
    mut commands: Commands,
    query: Query<&Player>,
    mut game_config: ResMut<GameConfiguration>,
) {
    if !query.is_empty() {
        return;
    }

    let generator = game_config.generator(false);
    let map = generator.generate_top_down_map().unwrap();

    let spawns = Spawns {
        map_spawn: Some(MapSpawn { map }),
        ..Default::default()
    };
    commands.spawn(spawns);
}
