use bevy::{
    input::keyboard::KeyboardInput,
    prelude::{
        AssetServer, Color, Commands, Entity, EventReader, Mut, Query, Rect, Res, ResMut, State,
        TextBundle,
    },
    text::{Text, TextStyle},
    ui::{AlignSelf, PositionType, Style, Val},
};

use crate::{
    components::{
        inventory::{InventoryText, InventoryTextBundle, InventoryTextStatus},
        player::{Player, PlayerInventory},
    },
    configuration::{crops::CropsConfig, game::GameConfiguration, tools::ToolConfigurations},
    states::AppState,
};

const PADDING: f32 = 15.0;
const INVENTORY_ITEM_SIZE: f32 = 50.0;
const FONT_SIZE: f32 = 20.0;

pub fn add_inventory_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_config: Res<GameConfiguration>,
    tool_configurations: Res<ToolConfigurations>,
) {
    commands.spawn_bundle(InventoryTextBundle {
        inventory_text: InventoryText,
        status: InventoryTextStatus {
            key: "title".to_string(),
        },
        text: TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(PADDING),
                    left: Val::Px(PADDING),
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
                    font_size: FONT_SIZE,
                    color: Color::WHITE,
                },
                // Note: You can use `Default::default()` in place of the `TextAlignment`
                Default::default(),
            ),
            ..Default::default()
        },
    });

    for (index, crop_config) in game_config.crops_config.configurations.iter().enumerate() {
        if crop_config.stages.is_empty() {
            continue;
        }

        let top = PADDING + (INVENTORY_ITEM_SIZE * (index as f32 + 1.0));
        commands.spawn_bundle(InventoryTextBundle {
            inventory_text: InventoryText,
            status: InventoryTextStatus {
                key: crop_config.key.clone(),
            },
            text: TextBundle {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    position_type: PositionType::Absolute,
                    position: Rect {
                        top: Val::Px(top),
                        left: Val::Px(PADDING),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                // Use the `Text::with_section` constructor
                text: Text::with_section(
                    // Accepts a `String` or any type that converts into a `String`, such as `&str`
                    format!(
                        "{}   {}",
                        crop_config.inventory_selector.display_code, crop_config.name
                    ),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: FONT_SIZE,
                        color: Color::WHITE,
                    },
                    // Note: You can use `Default::default()` in place of the `TextAlignment`
                    Default::default(),
                ),
                ..Default::default()
            },
        });
    }

    let count_crop_configs = game_config.crops_config.configurations.len();

    for (index, tool_config) in tool_configurations.configurations.iter().enumerate() {
        let top = PADDING + (INVENTORY_ITEM_SIZE * ((index + count_crop_configs) as f32 + 1.0));
        commands.spawn_bundle(InventoryTextBundle {
            inventory_text: InventoryText,
            status: InventoryTextStatus {
                key: tool_config.key.clone(),
            },
            text: TextBundle {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    position_type: PositionType::Absolute,
                    position: Rect {
                        top: Val::Px(top),
                        left: Val::Px(PADDING),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                // Use the `Text::with_section` constructor
                text: Text::with_section(
                    // Accepts a `String` or any type that converts into a `String`, such as `&str`
                    format!(
                        "{}   {}",
                        tool_config.inventory_selector.display_code, tool_config.name
                    ),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: FONT_SIZE,
                        color: Color::WHITE,
                    },
                    // Note: You can use `Default::default()` in place of the `TextAlignment`
                    Default::default(),
                ),
                ..Default::default()
            },
        });
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
    let (_, inventory): (&Player, &PlayerInventory) = match inventory_query.single() {
        Ok(it) => it,
        _ => return,
    };

    for text_data in text_query.iter_mut() {
        let (_, status, mut text): (&InventoryText, &InventoryTextStatus, Mut<Text>) = text_data;
        let section = match text.sections.get_mut(0) {
            Some(it) => it,
            _ => return,
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
    event_reader: EventReader<KeyboardInput>,
    state: ResMut<State<AppState>>,
    game_config: Res<GameConfiguration>,
    tool_configurations: Res<ToolConfigurations>,
    mut query: Query<(&Player, &mut PlayerInventory)>,
) {
    if state.current().ne(&AppState::InventoryScreen) {
        return;
    }

    let (_, inventory): (&Player, Mut<PlayerInventory>) = query.single_mut().unwrap();

    set_item_selection(
        inventory,
        &game_config.crops_config,
        tool_configurations,
        event_reader,
    );
}

fn set_item_selection(
    mut inventory: Mut<PlayerInventory>,
    crop_configurations: &CropsConfig,
    tool_configurations: Res<ToolConfigurations>,
    mut event_reader: EventReader<KeyboardInput>,
) {
    for event in event_reader.iter() {
        if let Some(crop_config) = crop_configurations
            .configurations
            .iter()
            .find(|config| config.inventory_selector.key_code == event.key_code.unwrap())
        {
            inventory.current_crop_config = Some(crop_config.clone());
            inventory.current_tool = None;
        } else if let Some(tool_config) = tool_configurations
            .configurations
            .iter()
            .find(|config| config.inventory_selector.key_code == event.key_code.unwrap())
        {
            inventory.current_tool = Some(tool_config.to_tool());
            inventory.current_crop_config = None;
        }
    }
}
