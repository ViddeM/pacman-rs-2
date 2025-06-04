use crate::{ghosts::ghost_mode::GhostModeRes, score::Score};
use bevy::prelude::*;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct DebugText;

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let text_font = TextFont {
        font: asset_server.load("fonts/Joystix.ttf"),
        font_size: 67.0,
        ..default()
    };

    // Score Text
    commands.spawn((
        Text::new("00"),
        ScoreText,
        text_font.clone(),
        TextShadow::default(),
        TextLayout::new_with_justify(JustifyText::Right),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(100.),
            left: Val::Px(80.),
            ..default()
        },
    ));

    let text_font = text_font.with_font_size(22.);

    // Debug Text
    commands.spawn((
        Text::new(""),
        DebugText,
        text_font.clone(),
        TextShadow::default(),
        TextLayout::new_with_justify(JustifyText::Right),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(100.),
            right: Val::Px(80.),
            ..default()
        },
    ));
}

pub fn update_score_text(score: Res<Score>, mut score_text: Single<&mut Text, With<ScoreText>>) {
    score_text.0 = format!("{:02}", score.score);
}

pub fn update_debug_text(
    ghost_mode: Res<GhostModeRes>,
    mut debug_text: Single<&mut Text, With<DebugText>>,
) {
    debug_text.0 = format!("DEBUG :: {:?} (H to toggle)", ghost_mode.global_mode);
}
