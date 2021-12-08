use bevy::prelude::{Commands, Res, ResMut};

use crate::{configuration::game::GameConfiguration, states::GameLoadState};

pub fn generate_world_grid(
    mut commands: Commands,
    mut load_state: ResMut<GameLoadState>,
    game_config: Res<GameConfiguration>,
) {
    let generator = game_config.world_config.generator(game_config.seed.clone());
    let world = generator.generate_top_down_map().unwrap();
    commands.insert_resource(world);

    load_state.game_world_generated = true;
}
