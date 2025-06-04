use bevy::prelude::*;

pub mod blinky;
pub mod clyde;
pub mod ghost_debug;
pub mod ghost_mode;
pub mod ghost_movement;
pub mod inky;
pub mod pinky;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GhostName {
    Blinky,
    Inky,
    Pinky,
    Clyde,
}

impl GhostName {
    fn get_color(&self) -> Color {
        match self {
            GhostName::Blinky => Color::linear_rgb(1.0, 0., 0.),
            GhostName::Inky => Color::linear_rgb(0., 0.8, 0.8),
            GhostName::Pinky => Color::linear_rgb(0.8, 0., 0.8),
            GhostName::Clyde => Color::linear_rgb(0.8, 0.8, 0.),
        }
    }
}
