use bevy::{
    input::keyboard::KeyboardInput,
    prelude::{
        AssetServer, Color, Commands, Entity, EventReader, KeyCode, Mut, Query, Rect, Res, ResMut,
        State, TextBundle,
    },
    text::{Text, TextStyle},
    ui::{AlignSelf, PositionType, Style, Val},
};

use crate::{
    components::{
        inventory::{InventoryText, InventoryTextBundle, InventoryTextStatus},
        player::{Player, PlayerInventory},
    },
    configuration::crops::CropConfigurations,
    states::{AppState, InventoryState},
};

pub fn add_inventory_text(
    mut commands: Commands,
    mut inventory_state: ResMut<InventoryState>,
    asset_server: Res<AssetServer>,
    crop_configurations: Res<CropConfigurations>,
) {
    if inventory_state.inventory_text.is_some() {
        return;
    }

    let mut inventory_text: Vec<Entity> = Vec::new();

    let title_entity = commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(15.0),
                    left: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            // Use the `Text::with_section` constructor
            text: Text::with_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "inventory",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
                // Note: You can use `Default::default()` in place of the `TextAlignment`
                Default::default(),
            ),
            ..Default::default()
        })
        .id();
    inventory_text.push(title_entity);

    for (index, crop_config) in crop_configurations.configurations.iter().enumerate() {
        if crop_config.sprite_index.is_none() {
            continue;
        }
        let top = 15.0 + (50.0 * (index as f32 + 1.0));
        let text_entity = commands
            .spawn_bundle(InventoryTextBundle {
                inventory_text: InventoryText,
                status: InventoryTextStatus { index },
                text: TextBundle {
                    style: Style {
                        align_self: AlignSelf::FlexEnd,
                        position_type: PositionType::Absolute,
                        position: Rect {
                            top: Val::Px(top),
                            left: Val::Px(15.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    // Use the `Text::with_section` constructor
                    text: Text::with_section(
                        // Accepts a `String` or any type that converts into a `String`, such as `&str`
                        format!("{} {}", index + 1, crop_config.name),
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 20.0,
                            color: Color::WHITE,
                        },
                        // Note: You can use `Default::default()` in place of the `TextAlignment`
                        Default::default(),
                    ),
                    ..Default::default()
                },
            })
            .id();
        inventory_text.push(text_entity);
    }

    inventory_state.inventory_text = Some(inventory_text);
}

pub fn remove_inventory_text(mut commands: Commands, mut inventory_state: ResMut<InventoryState>) {
    if inventory_state.inventory_text.is_none() {
        return;
    }

    if let Some(entities) = inventory_state.inventory_text.clone() {
        for entity in entities {
            commands.entity(entity).despawn();
        }
    }

    inventory_state.inventory_text = None;
}

pub fn update_inventory_text_colour(
    inventory_query: Query<(&Player, &PlayerInventory)>,
    mut text_query: Query<(&InventoryText, &InventoryTextStatus, &mut Text)>,
) {
    let (_, inventory): (&Player, &PlayerInventory) = inventory_query.single().unwrap();
    for text_data in text_query.iter_mut() {
        let (_, status, mut text): (&InventoryText, &InventoryTextStatus, Mut<'_, Text>) =
            text_data;
        if let Some(selected_index) = inventory.current_crop_selection {
            if selected_index == status.index {
                let section = text.sections.get_mut(0).unwrap();
                section.style.color = Color::YELLOW;
            } else {
                let section = text.sections.get_mut(0).unwrap();
                section.style.color = Color::WHITE;
            }
        }
    }
}

pub fn select_crop(
    event_reader: EventReader<KeyboardInput>,
    state: ResMut<State<AppState>>,
    crop_configurations: Res<CropConfigurations>,
    mut query: Query<(&Player, &mut PlayerInventory)>,
) {
    if state.current().ne(&AppState::InventoryScreen) {
        return;
    }

    let (_, inventory): (&Player, Mut<'_, PlayerInventory>) = query.single_mut().unwrap();

    let index_result = pressed_key_to_index(event_reader);

    if let Some(index) = index_result {
        set_crop_selection(inventory, crop_configurations, index);
    }
}

fn pressed_key_to_index(mut event_reader: EventReader<KeyboardInput>) -> Option<usize> {
    for event in event_reader.iter() {
        match event.key_code {
            Some(KeyCode::Numpad1) | Some(KeyCode::Key1) => return Some(0),
            Some(KeyCode::Numpad2) | Some(KeyCode::Key2) => return Some(1),
            Some(KeyCode::Numpad3) | Some(KeyCode::Key3) => return Some(2),
            Some(KeyCode::Numpad4) | Some(KeyCode::Key4) => return Some(3),
            Some(KeyCode::Numpad5) | Some(KeyCode::Key5) => return Some(4),
            Some(KeyCode::Numpad6) | Some(KeyCode::Key6) => return Some(5),
            Some(KeyCode::Numpad7) | Some(KeyCode::Key7) => return Some(6),
            Some(KeyCode::Numpad8) | Some(KeyCode::Key8) => return Some(7),
            Some(KeyCode::Numpad9) | Some(KeyCode::Key9) => return Some(8),
            Some(KeyCode::Numpad0) | Some(KeyCode::Key0) => return Some(9),
            _ => {}
        }
    }

    return None;
}

fn set_crop_selection(
    mut inventory: Mut<'_, PlayerInventory>,
    crop_configurations: Res<'_, CropConfigurations>,
    index: usize,
) {
    let config = crop_configurations.configurations.get(index);
    if config.is_some() {
        inventory.current_crop_selection = Some(index);
    }
}
