use bevy::prelude::{IntoSystem, ParallelSystemDescriptorCoercion, Plugin, SystemSet};

use crate::states::AppState;

use self::systems::{
    spawn::spawn,
    tick::{check_world_actions, tick_game_world},
};

pub mod components;
mod systems;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(
            SystemSet::on_exit(AppState::FinishedLoading).with_system(spawn.system()),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(tick_game_world.system().label("tick_game_world"))
                .with_system(
                    check_world_actions
                        .system()
                        .label("check_world_actions")
                        .after("tick_game_world"),
                ),
        );
    }
}
