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
    configuration::{crops::CropConfigurations, tools::ToolConfigurations},
    states::AppState,
};

pub fn add_inventory_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    crop_configurations: Res<CropConfigurations>,
    tool_configurations: Res<ToolConfigurations>,
) {
    commands
        .spawn_bundle(InventoryTextBundle {
            inventory_text: InventoryText,
            status: InventoryTextStatus { index: 99 },
            text: TextBundle {
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
            },
        })
        .id();

    for (index, crop_config) in crop_configurations.configurations.iter().enumerate() {
        if crop_config.stages.is_empty() {
            continue;
        }

        let top = 15.0 + (50.0 * (index as f32 + 1.0));
        commands
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
    }

    let count_crop_configs = crop_configurations.configurations.len();

    for (index, tool_config) in tool_configurations.configurations.iter().enumerate() {
        let text_entry_index = index + count_crop_configs;
        let top = 15.0 + (50.0 * (text_entry_index as f32 + 1.0));
        commands
            .spawn_bundle(InventoryTextBundle {
                inventory_text: InventoryText,
                status: InventoryTextStatus {
                    index: text_entry_index,
                },
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
                        format!("{} {}", text_entry_index + 1, tool_config.name),
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
    }
}

pub fn remove_inventory_text(mut commands: Commands, query: Query<(&InventoryText, Entity)>) {
    for data in query.iter() {
        let (_, entity): (&InventoryText, Entity) = data;
        commands.entity(entity).despawn();
    }
}

pub fn update_inventory_text_colour(
    inventory_query: Query<(&Player, &PlayerInventory)>,
    mut text_query: Query<(&InventoryText, &InventoryTextStatus, &mut Text)>,
) {
    let (_, inventory): (&Player, &PlayerInventory) = inventory_query.single().unwrap();
    for text_data in text_query.iter_mut() {
        let (_, status, mut text): (&InventoryText, &InventoryTextStatus, Mut<'_, Text>) =
            text_data;
        if inventory.current_tool.is_some() {
            if let Some(selected_tool_index) = inventory.current_tool_selection {
                if selected_tool_index == status.index {
                    let section = text.sections.get_mut(0).unwrap();
                    section.style.color = Color::YELLOW;
                } else {
                    let section = text.sections.get_mut(0).unwrap();
                    section.style.color = Color::WHITE;
                }
            }
        } else if let Some(selected_index) = inventory.current_crop_selection {
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
    tool_configurations: Res<ToolConfigurations>,
    mut query: Query<(&Player, &mut PlayerInventory)>,
) {
    if state.current().ne(&AppState::InventoryScreen) {
        return;
    }

    let (_, inventory): (&Player, Mut<'_, PlayerInventory>) = query.single_mut().unwrap();

    let index_result = pressed_key_to_index(event_reader);

    if let Some(index) = index_result {
        set_item_selection(inventory, crop_configurations, tool_configurations, index);
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

    None
}

fn set_item_selection(
    mut inventory: Mut<'_, PlayerInventory>,
    crop_configurations: Res<'_, CropConfigurations>,
    tool_configurations: Res<'_, ToolConfigurations>,
    index: usize,
) {
    if index >= crop_configurations.configurations.len() {
        let tool_index = index - crop_configurations.configurations.len();
        if let Some(tool_config) = tool_configurations.configurations.get(tool_index) {
            inventory.current_tool = Some(tool_config.to_tool());
            inventory.current_tool_selection = Some(index);
            inventory.current_crop_selection = None;
        }
    } else {
        let config = crop_configurations.configurations.get(index);
        if config.is_some() {
            inventory.current_tool = None;
            inventory.current_crop_selection = None;
            inventory.current_crop_selection = Some(index);
        }
    }
}
