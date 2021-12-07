use bevy::{
    input::Input,
    prelude::{Commands, KeyCode, Mut, Query, Res, ResMut, State},
};

use crate::{
    components::player::PlayerInventory,
    plugins::inventory::components::selection::CurrentInventorySelection, states::AppState,
};

pub fn add_current_selection(mut commands: Commands, query: Query<&CurrentInventorySelection>) {
    if !query.is_empty() {
        return;
    }

    commands
        .spawn()
        .insert(CurrentInventorySelection::default());
}

pub fn selection_input(
    keyboard_input: Res<Input<KeyCode>>,
    state: ResMut<State<AppState>>,
    mut query: Query<&mut CurrentInventorySelection>,
    current_inventory_query: Query<&PlayerInventory>,
) {
    if state.current().ne(&AppState::InventoryScreen) {
        return;
    }

    let mut current_selection: Mut<'_, CurrentInventorySelection> = query.single_mut();
    let current_inventory: &PlayerInventory = current_inventory_query.single();

    if keyboard_input.just_pressed(KeyCode::Down) {
        let new_index = match current_inventory.current_selected_index {
            Some(it) => {
                if it + 1 > current_selection.max_index {
                    0
                } else {
                    it + 1
                }
            }
            None => 0,
        };
        current_selection.index = Some(new_index);
        return;
    }

    if keyboard_input.just_pressed(KeyCode::Up) {
        let new_index = match current_inventory.current_selected_index {
            Some(it) => {
                if it == 0 {
                    current_selection.max_index
                } else {
                    it - 1
                }
            }
            None => 0,
        };
        current_selection.index = Some(new_index);
        return;
    }

    for key in keyboard_input.get_just_pressed() {
        current_selection.key_code = Some(*key);
    }
}

pub fn reset_selection(mut query: Query<&mut CurrentInventorySelection>) {
    let mut selection: Mut<'_, CurrentInventorySelection> = query.single_mut();
    selection.index = None;
    selection.key_code = None;
}

pub fn select_item(
    state: ResMut<State<AppState>>,
    mut query: Query<&mut PlayerInventory>,
    selection_query: Query<&CurrentInventorySelection>,
) {
    if state.current().ne(&AppState::InventoryScreen) {
        return;
    }

    let selection: &CurrentInventorySelection = selection_query.single();

    if selection.index.is_none() && selection.key_code.is_none() {
        return;
    }

    let mut inventory: Mut<PlayerInventory> = query.single_mut();

    if let Some(index) = selection.index {
        inventory.current_selected_index = Some(index);
        if index > inventory.held_seeds.len() - 1 {
            let tool_index = index - inventory.held_seeds.len();
            if let Some(tool_config) = inventory.held_tools.get(tool_index) {
                inventory.current_tool = Some(tool_config.to_tool());
                inventory.current_crop_config = None;
            }
        } else if let Some(seed_config) = inventory.held_seeds.get(index) {
            inventory.current_crop_config = Some(seed_config.clone());
            inventory.current_tool = None;
        }
    } else if let Some(key_code) = selection.key_code {
        if let Some((i, crop_config)) = inventory
            .held_seeds
            .iter()
            .enumerate()
            .find(|(_, config)| config.inventory_selector.key_code == key_code)
        {
            inventory.current_crop_config = Some(crop_config.clone());
            inventory.current_selected_index = Some(i);
            inventory.current_tool = None;
        } else if let Some((i, tool_config)) = inventory
            .held_tools
            .iter()
            .enumerate()
            .find(|(_, config)| config.inventory_selector().key_code == key_code)
        {
            inventory.current_tool = Some(tool_config.to_tool());
            inventory.current_selected_index = Some(i + (inventory.held_seeds.len()));
            inventory.current_crop_config = None;
        }
    }
}
