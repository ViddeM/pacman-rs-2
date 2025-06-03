use bevy::prelude::*;

use crate::{
    common::{Direction, PixelPos, TilePos},
    ghosts::GhostName,
    score::Scorable,
};

pub const FULL_SPEED_PIXELS_PER_SECOND: f32 = 75.75757625;

#[derive(Component)]
pub struct Player;

#[derive(Component, Deref, DerefMut, PartialEq, Debug, Clone)]
pub struct Position(pub PixelPos);

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct GhostDoor;

#[derive(Component)]
pub struct Ghost {
    pub ghost: GhostName,
}

#[derive(Component)]
pub struct GhostDebug {
    pub ghost: GhostName,
}

#[derive(Component)]
pub struct QueableDirection {
    pub next_direction: Option<Direction>,
}

#[derive(Component)]
pub struct GhostTarget {
    pub tile: Option<TilePos>,
}

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct Movable {
    pub direction: Direction,
    /// Progress to the next pixel in the direction of travel.
    pub progress: f32,
    pub target_tile: TilePos,
    /// The speed of this movable, acts as a percentage where 100% (1.0) = FULL_SPEED_PIXELS_PER_SECOND pixels/sec.
    pub speed: f32,
    /// The amount of 'frames' to pause for.
    pub pause_frames: Option<u32>,
    pub pause_time: f32,
}

impl Movable {
    pub fn new(target: TilePos, direction: Direction, speed: f32) -> Self {
        Self {
            direction,
            speed,
            progress: 0.,
            target_tile: target,
            pause_frames: None,
            pause_time: 0.,
        }
    }

    pub fn pause(&mut self, scorable: &Scorable) {
        self.pause_frames = scorable.pause_frames();
        self.pause_time = 0.;
    }

    pub fn reduce_pause_time(&mut self) {
        let prev = self.pause_frames.unwrap();
        let new = prev - 1;
        if new == 0 {
            self.pause_frames = None;
        } else {
            self.pause_frames = Some(new);
        }
    }
}

#[derive(Component)]
pub struct AnimationIndices {
    current_index: usize,
    sprite_indices_right: Vec<usize>,
    sprite_indices_left: Vec<usize>,
    sprite_indices_up: Vec<usize>,
    sprite_indices_down: Vec<usize>,
}

impl AnimationIndices {
    pub fn new(right: Vec<usize>, left: Vec<usize>, up: Vec<usize>, down: Vec<usize>) -> Self {
        Self {
            current_index: 0,
            sprite_indices_right: right,
            sprite_indices_left: left,
            sprite_indices_up: up,
            sprite_indices_down: down,
        }
    }

    pub fn next(&mut self, dir: &Direction) -> usize {
        let curr_indices = match dir {
            Direction::Up => &self.sprite_indices_up,
            Direction::Right => &self.sprite_indices_right,
            Direction::Down => &self.sprite_indices_down,
            Direction::Left => &self.sprite_indices_left,
        };

        self.current_index = (self.current_index + 1) % curr_indices.len();

        curr_indices[self.current_index]
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);
