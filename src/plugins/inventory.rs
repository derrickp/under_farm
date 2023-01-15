mod components;
mod systems;
use bevy::prelude::*;

use crate::states::AppState;

use self::systems::camera::{hide_game_sprites, remove_ui_camera, show_game_sprites};
use self::systems::{
    camera::remove_gameplay_camera,
    input::open_close_inventory_input_system,
    selection::{add_current_selection, reset_selection, select_item, selection_input},
    text::{add_text, remove_text, update_text_colour},
};

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(open_close_inventory_input_system)
            .add_system_set(
                SystemSet::on_enter(AppState::InGame).with_system(add_current_selection),
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
            );
    }
}

#[derive(SystemLabel, Debug, Clone, PartialEq, Eq, Hash)]
enum Label {
    ResetInventorySelection,
    SelectItem,
    InventoryInput,
}
