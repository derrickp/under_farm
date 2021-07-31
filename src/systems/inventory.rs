use bevy::{
    prelude::{
        AssetServer, Color, Commands, Entity, OrthographicCameraBundle, Rect, Res, ResMut,
        TextBundle,
    },
    text::{Text, TextStyle},
    ui::{AlignSelf, PositionType, Style, Val},
};

use crate::{
    components::camera::GameCamera,
    configuration::crops::CropConfigurations,
    states::{GameState, InventoryState},
};

pub fn remove_gameplay_camera(mut commands: Commands, mut game_state: ResMut<GameState>) {
    if let Some(camera_entity) = game_state.game_camera {
        commands.entity(camera_entity).despawn();
        game_state.game_camera = None;
    }
}

pub fn add_gameplay_camera(mut commands: Commands, mut game_state: ResMut<GameState>) {
    if let None = game_state.game_camera {
        let camera = commands
            .spawn_bundle(OrthographicCameraBundle::new_2d())
            .insert(GameCamera)
            .id();
        game_state.game_camera = Some(camera);
    }
}

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
        let top = 15.0 + (50.0 * (index as f32 + 1.0));
        let text_entity = commands
            .spawn_bundle(TextBundle {
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
