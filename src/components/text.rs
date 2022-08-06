use bevy::{
    prelude::{AssetServer, Bundle, Color, Component, Res, TextBundle},
    text::{Text, TextStyle},
    ui::{AlignSelf, PositionType, Style, UiRect, Val},
    window::Windows,
};
use tdlg::coordinate::Coordinate;

#[derive(Component)]
pub struct PlayerStatsText;

#[derive(Bundle)]
pub struct PlayerStatsTextBundle {
    pub stats_text: PlayerStatsText,

    #[bundle]
    pub text: TextBundle,
}

impl PlayerStatsTextBundle {
    pub fn build(
        coordinate: &Coordinate,
        asset_server: &Res<AssetServer>,
        windows: &Res<Windows>,
    ) -> Self {
        let window = windows.get_primary().unwrap();
        let x = window.width() * 0.87;
        let y = window.height() * 0.95;

        Self {
            stats_text: PlayerStatsText,
            text: TextBundle {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        top: Val::Px(y),
                        left: Val::Px(x),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                // Use the `Text::with_section` constructor
                text: Text::from_section(
                    // Accepts a `String` or any type that converts into a `String`, such as `&str`
                    format!("Coordinate {}  {}", coordinate.x, coordinate.y),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                ),
                ..Default::default()
            },
        }
    }
}
