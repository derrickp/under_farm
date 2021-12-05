use bevy::{
    input::keyboard::KeyboardInput,
    prelude::{
        AssetServer, Color, Commands, Entity, EventReader, Handle, Mut, Query, Res, ResMut, State,
    },
    text::{Font, Text},
};

use crate::{
    components::{
        inventory::{InventoryText, InventoryTextBundle, InventoryTextStatus},
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
) {
    let font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");

    let title_bundle = InventoryTextBundle::build(
        &"title".to_string(),
        PADDING,
        PADDING,
        "inventory".to_string(),
        &font,
        FONT_SIZE,
    );
    commands.spawn_bundle(title_bundle);
    let player_inventory: &PlayerInventory = query.single();

    let mut seed_count = 0;
    for (index, crop_config) in player_inventory.held_seeds.iter().enumerate() {
        if crop_config.stages.is_empty() {
            continue;
        }

        seed_count += 1;
        let top = PADDING + (INVENTORY_ITEM_SIZE * (index as f32 + 1.0));
        let text_bundle = InventoryTextBundle::build(
            &crop_config.key,
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
    }

    for (index, tool_config) in player_inventory.held_tools.iter().enumerate() {
        let top = PADDING + (INVENTORY_ITEM_SIZE * ((index + seed_count) as f32 + 1.0));
        let text_bundle = InventoryTextBundle::build(
            tool_config.key(),
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
    }
}

pub fn remove_inventory_text(mut commands: Commands, query: Query<(&InventoryText, Entity)>) {
    for data in query.iter() {
        let (_, entity): (&InventoryText, Entity) = data;
        commands.entity(entity).despawn();
    }
}

pub fn update_inventory_text_colour(
    inventory_query: Query<&PlayerInventory>,
    mut text_query: Query<(&InventoryText, &InventoryTextStatus, &mut Text)>,
) {
    if inventory_query.is_empty() {
        return;
    }

    let inventory: &PlayerInventory = inventory_query.single();

    for text_data in text_query.iter_mut() {
        let (_, status, mut text): (&InventoryText, &InventoryTextStatus, Mut<Text>) = text_data;
        let section = match text.sections.get_mut(0) {
            Some(it) => it,
            _ => continue,
        };

        if let Some(tool) = &inventory.current_tool {
            if tool.key.eq(&status.key) {
                section.style.color = Color::YELLOW;
            } else {
                section.style.color = Color::WHITE;
            }
        } else if let Some(crop_config) = &inventory.current_crop_config {
            if crop_config.key.eq(&status.key) {
                section.style.color = Color::YELLOW;
            } else {
                section.style.color = Color::WHITE;
            }
        }
    }
}

pub fn select_item(
    mut event_reader: EventReader<KeyboardInput>,
    state: ResMut<State<AppState>>,
    mut query: Query<&mut PlayerInventory>,
) {
    if state.current().ne(&AppState::InventoryScreen) {
        return;
    }

    let mut inventory: Mut<PlayerInventory> = query.single_mut();
    for event in event_reader.iter() {
        if let Some(crop_config) = inventory
            .held_seeds
            .iter()
            .find(|config| config.inventory_selector.key_code == event.key_code.unwrap())
        {
            inventory.current_crop_config = Some(crop_config.clone());
            inventory.current_tool = None;
        } else if let Some(tool_config) = inventory
            .held_tools
            .iter()
            .find(|config| config.inventory_selector().key_code == event.key_code.unwrap())
        {
            inventory.current_tool = Some(tool_config.to_tool());
            inventory.current_crop_config = None;
        }
    }
}
