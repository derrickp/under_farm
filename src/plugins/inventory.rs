mod components;
mod systems;

use bevy::prelude::{IntoSystem, ParallelSystemDescriptorCoercion, Plugin, SystemSet};

use crate::states::AppState;

use self::systems::{
    camera::remove_gameplay_camera,
    input::open_close_inventory_input_system,
    selection::{add_current_selection, reset_selection, select_item, selection_input},
    text::{add_text, remove_text, update_text_colour},
};

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(open_close_inventory_input_system.system())
            .add_system_set(
                SystemSet::on_enter(AppState::InGame).with_system(add_current_selection.system()),
            )
            .add_system_set(
                SystemSet::on_enter(AppState::InventoryScreen)
                    .with_system(add_text.system())
                    .with_system(remove_gameplay_camera.system()),
            )
            .add_system_set(
                SystemSet::on_exit(AppState::InventoryScreen).with_system(remove_text.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InventoryScreen)
                    .with_system(selection_input.system().label("inventory_input"))
                    .with_system(
                        select_item
                            .system()
                            .after("inventory_input")
                            .label("select_item"),
                    )
                    .with_system(update_text_colour.system().after("inventory_input"))
                    .with_system(
                        reset_selection
                            .system()
                            .label("reset_inventory_selection")
                            .after("select_item"),
                    ),
            );
    }
}
