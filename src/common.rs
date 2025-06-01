use crate::map::{MAP_HEIGHT, MAP_WIDTH, TILE_SIZE};

#[derive(PartialEq, Clone, Debug)]
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

#[derive(Clone, Debug, PartialEq)]
pub struct PixelPos {
    pub x: i32,
    pub y: i32,
}

impl PixelPos {
    pub fn dist_to(&self, other: &PixelPos) -> f32 {
        (((self.x - other.x).abs() + (self.y - other.y).abs()) as f32).sqrt()
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
            x: value.x * TILE_SIZE,
            y: value.y * TILE_SIZE,
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
        self.x % TILE_SIZE == 0 && self.y % TILE_SIZE == 0
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

        let new_x = (self.x as i32 + translate_x).max(0).min(MAP_WIDTH as i32);
        let new_y = (self.y as i32 + translate_y).max(0).min(MAP_HEIGHT as i32);
        Self { x: new_x, y: new_y }
    }

    pub fn dist_to(&self, other: &TilePos) -> f32 {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as f32 / 2.0
    }
}
