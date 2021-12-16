use bevy::prelude::{Commands, ResMut};

use crate::{configuration::game::GameConfiguration, states::GameLoadState};

pub fn generate_world_grid(
    mut commands: Commands,
    mut load_state: ResMut<GameLoadState>,
    mut game_config: ResMut<GameConfiguration>,
) {
    let generator = game_config.generator(false);
    let world = generator.generate_top_down_map().unwrap();
    commands.insert_resource(world);
    commands.insert_resource(generator);

    load_state.game_world_generated = true;
}
