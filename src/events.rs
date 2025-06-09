use bevy::prelude::*;

use crate::{
    common::{Character, TilePos},
    ghosts::GhostName,
};

#[derive(Event)]
pub struct CharacterReachedTargetEvent {
    pub character: Character,
    pub tile: TilePos,
}

impl CharacterReachedTargetEvent {
    pub fn new(character: Character, tile: TilePos) -> Self {
        CharacterReachedTargetEvent { character, tile }
    }

    pub fn is_pacman(&self) -> bool {
        self.character == Character::Pacman
    }

    pub fn is_blinky(&self) -> bool {
        self.character
            == Character::Ghost {
                name: GhostName::Blinky,
            }
    }

    pub fn is_pinky(&self) -> bool {
        self.character
            == Character::Ghost {
                name: GhostName::Pinky,
            }
    }

    pub fn is_inky(&self) -> bool {
        self.character
            == Character::Ghost {
                name: GhostName::Inky,
            }
    }

    pub fn is_clyde(&self) -> bool {
        self.character
            == Character::Ghost {
                name: GhostName::Clyde,
            }
    }
}
