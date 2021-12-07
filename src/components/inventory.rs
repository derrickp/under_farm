use bevy::{
    math::Rect,
    prelude::{Bundle, Color, Component, Handle, KeyCode, TextBundle},
    text::{Font, Text, TextStyle},
    ui::{AlignSelf, PositionType, Style, Val},
};

#[derive(Component)]
pub struct InventoryText {
    pub selection_helper: Option<InventorySelectionHelper>,
}

pub struct InventorySelectionHelper {
    pub key: String,
    pub index: usize,
}

#[derive(Bundle)]
pub struct InventoryTextBundle {
    pub inventory_text: InventoryText,

    #[bundle]
    pub text: TextBundle,
}

impl InventoryTextBundle {
    pub fn build(
        helper: Option<InventorySelectionHelper>,
        top: f32,
        left: f32,
        text: String,
        font: &Handle<Font>,
        font_size: f32,
    ) -> Self {
        Self {
            inventory_text: InventoryText {
                selection_helper: helper,
            },
            text: TextBundle {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    position_type: PositionType::Absolute,
                    position: Rect {
                        top: Val::Px(top),
                        left: Val::Px(left),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                // Use the `Text::with_section` constructor
                text: Text::with_section(
                    // Accepts a `String` or any type that converts into a `String`, such as `&str`
                    text,
                    TextStyle {
                        font: font.clone(),
                        font_size,
                        color: Color::WHITE,
                    },
                    // Note: You can use `Default::default()` in place of the `TextAlignment`
                    Default::default(),
                ),
                ..Default::default()
            },
        }
    }
}

#[derive(Clone)]
pub struct InventorySelector {
    pub key_code: KeyCode,
    pub display_code: String,
}

#[derive(Default, Component)]
pub struct CurrentInventorySelection {
    pub key_code: Option<KeyCode>,
    pub index: Option<usize>,
    pub max_index: usize,
}
