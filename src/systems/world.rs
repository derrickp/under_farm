use bevy::{
    core::Time,
    prelude::{Commands, Res, ResMut},
};
use tdlg::generator::Generator;

use crate::{states::GameLoadState, world::WorldTickTimer};

const DEFAULT_GRID_SIZE: usize = 150;
const NUMBER_OF_ROOMS: usize = 75;

pub fn tick_game_world(time: Res<Time>, mut timer: ResMut<WorldTickTimer>) {
    timer.0.tick(time.delta());
}

pub fn generate_world_grid(mut commands: Commands, mut load_state: ResMut<GameLoadState>) {
    let generator = Generator {
        grid_size: DEFAULT_GRID_SIZE,
        target_number_rooms: NUMBER_OF_ROOMS,
        room_template_directory: "assets/room_templates",
    };
    let world = generator.generate_top_down_map();
    println!("{}", world.room_count);
    commands.insert_resource(world.grid);

    load_state.game_world_generated = true;
}
