use bevy::prelude::*;

use crate::{components::GhostDebug, ui::DebugText};

#[derive(Resource)]
pub struct DebugRes {
    pub debug_mode: bool,
}

impl Default for DebugRes {
    fn default() -> Self {
        Self { debug_mode: false }
    }
}

pub fn toggle_debug_mode(
    mut debug: ResMut<DebugRes>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    ghost_debug_query: Query<&mut Transform, With<GhostDebug>>,
    mut debug_text: Single<&mut Text, With<DebugText>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyT) {
        debug.debug_mode = !debug.debug_mode;

        if !debug.debug_mode {
            debug_text.0 = "".into();
            for mut transform in ghost_debug_query {
                transform.translation = Vec3::new(-1000., -1000., 0.);
            }
        }
    }
}

pub fn run_if_debug(debug: Res<DebugRes>) -> bool {
    debug.debug_mode
}
