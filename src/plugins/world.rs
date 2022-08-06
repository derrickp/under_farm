use bevy::prelude::ParallelSystemDescriptorCoercion;
use bevy::prelude::SystemLabel;
use bevy::prelude::{Plugin, SystemSet};

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
        app.add_system_set(SystemSet::on_exit(AppState::FinishedLoading).with_system(spawn))
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(tick_game_world.label(Label::TickGameWorld))
                    .with_system(
                        check_world_actions
                            .label(Label::CheckWorldActions)
                            .after(Label::TickGameWorld),
                    ),
            );
    }
}

#[derive(SystemLabel, Debug, Clone, PartialEq, Eq, Hash)]
enum Label {
    CheckWorldActions,
    TickGameWorld,
}
