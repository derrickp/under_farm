use bevy::{
    prelude::{AssetServer, Color, Commands, Entity, Handle, Mut, Query, Res},
    text::{Font, Text},
};

use crate::components::{
    inventory::{
        CurrentInventorySelection, InventorySelectionHelper, InventoryText, InventoryTextBundle,
    },
    player::PlayerInventory,
};

const PADDING: f32 = 15.0;
const INVENTORY_ITEM_SIZE: f32 = 50.0;
const FONT_SIZE: f32 = 20.0;

pub fn add_text(
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
    commands.spawn(title_bundle);
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
        commands.spawn(text_bundle);

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
        commands.spawn(text_bundle);

        total_count += 1;
    }

    let mut current_selection: Mut<'_, CurrentInventorySelection> = selection_query.single_mut();
    current_selection.max_index = total_count - 1;
}

pub fn update_text_colour(
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

pub fn remove_text(mut commands: Commands, query: Query<(&InventoryText, Entity)>) {
    for data in query.iter() {
        let (_, entity): (&InventoryText, Entity) = data;
        commands.entity(entity).despawn();
    }
}
