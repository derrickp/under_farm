use bevy::{
    math::Rect,
    prelude::{AssetServer, Bundle, Color, Res, TextBundle},
    text::{Text, TextStyle},
    ui::{AlignSelf, PositionType, Style, Val},
    window::Windows,
};
use tdlg::coordinate::Coordinate;

use super::player::PlayerBundle;

pub struct PlayerStatsText;

#[derive(Bundle)]
pub struct PlayerStatsTextBundle {
    pub stats_text: PlayerStatsText,

    #[bundle]
    pub text: TextBundle,
}

impl PlayerStatsTextBundle {
    pub fn from_player_bundle(
        player_bundle: &PlayerBundle,
        asset_server: &Res<AssetServer>,
        windows: &Res<Windows>,
    ) -> Self {
        let current = match player_bundle.coordinates.current {
            Some(it) => it,
            _ => Coordinate::new(0, 0),
        };

        let window = windows.get_primary().unwrap();
        let x = window.width() * 0.87;
        let y = window.height() * 0.95;

        Self {
            stats_text: PlayerStatsText,
            text: TextBundle {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    position_type: PositionType::Absolute,
                    position: Rect {
                        top: Val::Px(y),
                        left: Val::Px(x),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                // Use the `Text::with_section` constructor
                text: Text::with_section(
                    // Accepts a `String` or any type that converts into a `String`, such as `&str`
                    format!("Coordinate {}  {}", current.x, current.y),
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
        }
    }
}
