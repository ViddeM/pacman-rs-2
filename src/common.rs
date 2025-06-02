use crate::map::{TILE_CENTER_PIXEL_OFFSET_X, TILE_CENTER_PIXEL_OFFSET_Y, TILE_SIZE};
use bevy::prelude::*;

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TilePos {
    pub x: i32,
    pub y: i32,
}

impl TilePos {
    pub fn to_maze_display_pos(&self) -> Vec3 {
        let pixel_pos: PixelPos = self.into();
        Vec3::new(pixel_pos.x as f32, -pixel_pos.y as f32, -1.)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PixelPos {
    pub x: i32,
    pub y: i32,
}

impl PixelPos {
    pub fn to_character_display_pos(&self) -> Vec3 {
        Vec3::new(
            (self.x - TILE_SIZE + TILE_CENTER_PIXEL_OFFSET_X) as f32,
            -(self.y - TILE_SIZE + TILE_CENTER_PIXEL_OFFSET_Y) as f32,
            0.,
        )
    }
}

impl From<TilePos> for PixelPos {
    fn from(value: TilePos) -> Self {
        (&value).into()
    }
}

impl From<&TilePos> for PixelPos {
    fn from(value: &TilePos) -> Self {
        PixelPos {
            x: value.x * TILE_SIZE + TILE_CENTER_PIXEL_OFFSET_X,
            y: value.y * TILE_SIZE + TILE_CENTER_PIXEL_OFFSET_Y,
        }
    }
}

impl From<PixelPos> for TilePos {
    fn from(value: PixelPos) -> Self {
        (&value).into()
    }
}

impl From<&PixelPos> for TilePos {
    fn from(value: &PixelPos) -> Self {
        let x = value.x;
        let y = value.y;
        let tile_x = (x - (x % TILE_SIZE)) / TILE_SIZE;
        let tile_y = (y - (y % TILE_SIZE)) / TILE_SIZE;
        TilePos {
            x: tile_x,
            y: tile_y,
        }
    }
}

impl PixelPos {
    pub fn in_middle_of_tile(&self) -> bool {
        self.x.abs() % TILE_SIZE == TILE_CENTER_PIXEL_OFFSET_X
            && self.y.abs() % TILE_SIZE == TILE_CENTER_PIXEL_OFFSET_Y
    }
}

impl TilePos {
    pub fn translate(&self, dir: &Direction) -> Self {
        let (translate_x, translate_y) = match dir {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        };

        let new_x = self.x as i32 + translate_x;
        let new_y = self.y as i32 + translate_y;
        Self { x: new_x, y: new_y }
    }

    pub fn dist_to(&self, other: &TilePos) -> f32 {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as f32 / 2.0
    }
}
