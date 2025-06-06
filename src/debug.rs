use bevy::{
    ecs::{
        resource::Resource,
        system::{Res, ResMut},
    },
    input::{ButtonInput, keyboard::KeyCode},
};

#[derive(Resource)]
pub struct DebugRes {
    pub debug_mode: bool,
}

impl Default for DebugRes {
    fn default() -> Self {
        Self { debug_mode: false }
    }
}

pub fn toggle_debug_mode(mut debug: ResMut<DebugRes>, keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::KeyT) {
        debug.debug_mode = !debug.debug_mode;
    }
}
