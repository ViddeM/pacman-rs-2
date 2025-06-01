use crate::{components::ScoreText, score::Score};
use bevy::prelude::*;

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Text::new("00"),
        ScoreText,
        TextFont {
            font: asset_server.load("fonts/Joystix.ttf"),
            font_size: 67.0,
            ..default()
        },
        TextShadow::default(),
        TextLayout::new_with_justify(JustifyText::Right),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(100.),
            left: Val::Px(80.),
            ..default()
        },
    ));
}

pub fn update_score_text(score: Res<Score>, mut score_text: Single<&mut Text, With<ScoreText>>) {
    score_text.0 = format!("{:02}", score.score);
}
