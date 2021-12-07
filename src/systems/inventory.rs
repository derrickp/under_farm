use bevy::{
    input::Input,
    prelude::{
        AssetServer, Color, Commands, Entity, Handle, KeyCode, Mut, Query, Res, ResMut, State,
    },
    text::{Font, Text},
};

use crate::{
    components::{
        inventory::{
            CurrentInventorySelection, InventorySelectionHelper, InventoryText, InventoryTextBundle,
        },
        player::PlayerInventory,
    },
    states::AppState,
};

const PADDING: f32 = 15.0;
const INVENTORY_ITEM_SIZE: f32 = 50.0;
const FONT_SIZE: f32 = 20.0;

pub fn add_inventory_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<&PlayerInventory>,
    mut selection_query: Query<&mut CurrentInventorySelection>,
) {
    let font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");

    let title_bundle = InventoryTextBundle::build(
        None,
        PADDING,
        PADDING,
        "inventory".to_string(),
        &font,
        FONT_SIZE,
    );
    commands.spawn_bundle(title_bundle);
    let player_inventory: &PlayerInventory = query.single();

    let mut seed_count = 0;
    let mut total_count = 0;
    for (index, crop_config) in player_inventory.held_seeds.iter().enumerate() {
        if crop_config.stages.is_empty() {
            continue;
        }

        seed_count += 1;
        let top = PADDING + (INVENTORY_ITEM_SIZE * (index as f32 + 1.0));
        let text_bundle = InventoryTextBundle::build(
            Some(InventorySelectionHelper {
                index,
                key: crop_config.key.to_string(),
            }),
            top,
            PADDING,
            format!(
                "{}   {}",
                crop_config.inventory_selector.display_code, crop_config.name
            ),
            &font,
            FONT_SIZE,
        );
        commands.spawn_bundle(text_bundle);

        total_count += 1;
    }

    for (index, tool_config) in player_inventory.held_tools.iter().enumerate() {
        let top = PADDING + (INVENTORY_ITEM_SIZE * ((index + seed_count) as f32 + 1.0));
        let text_bundle = InventoryTextBundle::build(
            Some(InventorySelectionHelper {
                key: tool_config.key().to_string(),
                index: index + seed_count,
            }),
            top,
            PADDING,
            format!(
                "{}   {}",
                tool_config.inventory_selector().display_code,
                tool_config.name()
            ),
            &font,
            FONT_SIZE,
        );
        commands.spawn_bundle(text_bundle);

        total_count += 1;
    }

    let mut current_selection: Mut<'_, CurrentInventorySelection> = selection_query.single_mut();
    current_selection.max_index = total_count - 1;
}

pub fn remove_inventory_text(mut commands: Commands, query: Query<(&InventoryText, Entity)>) {
    for data in query.iter() {
        let (_, entity): (&InventoryText, Entity) = data;
        commands.entity(entity).despawn();
    }
}

pub fn update_inventory_text_colour(
    inventory_query: Query<&PlayerInventory>,
    mut text_query: Query<(&InventoryText, &mut Text)>,
) {
    if inventory_query.is_empty() {
        return;
    }

    let inventory: &PlayerInventory = inventory_query.single();

    for text_data in text_query.iter_mut() {
        let (inventory_text, mut text): (&InventoryText, Mut<Text>) = text_data;
        let section = match text.sections.get_mut(0) {
            Some(it) => it,
            _ => continue,
        };

        let key = match &inventory_text.selection_helper {
            Some(it) => it.key.clone(),
            _ => continue,
        };

        if let Some(tool) = &inventory.current_tool {
            if tool.key.eq(&key) {
                section.style.color = Color::YELLOW;
            } else {
                section.style.color = Color::WHITE;
            }
        } else if let Some(crop_config) = &inventory.current_crop_config {
            if crop_config.key.eq(&key) {
                section.style.color = Color::YELLOW;
            } else {
                section.style.color = Color::WHITE;
            }
        }
    }
}

pub fn inventory_input(
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

pub fn reset_inventory_selection(mut query: Query<&mut CurrentInventorySelection>) {
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
