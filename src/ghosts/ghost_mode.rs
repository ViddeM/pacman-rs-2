use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GhostMode {
    Chase,
    Scatter,
}

impl GhostMode {
    pub fn next(&self) -> Self {
        match self {
            GhostMode::Chase => Self::Scatter,
            GhostMode::Scatter => Self::Chase,
        }
    }
}

#[derive(Resource)]
pub struct GhostModeRes {
    pub global_mode: GhostMode,
}

impl Default for GhostModeRes {
    fn default() -> Self {
        Self {
            global_mode: GhostMode::Chase,
        }
    }
}
