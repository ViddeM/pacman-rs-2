use bevy::prelude::*;

#[derive(Resource)]
pub struct Score {
    pub score: u32,
    pub ghosts_eaten: u32,
}

impl Score {
    pub fn new() -> Self {
        Self {
            score: 0,
            ghosts_eaten: 0,
        }
    }

    pub fn gain_score(&mut self, scorable: &Scorable) {
        let score = match scorable {
            Scorable::Dot => 10,
            Scorable::Energizer => 50,
            Scorable::Ghost => 200 * 2u32.pow(self.ghosts_eaten),
        };

        self.score += score;
    }
}

#[derive(Component)]
pub enum Scorable {
    Dot,
    Energizer,
    Ghost,
}

impl Scorable {
    pub fn pause_frames(&self) -> Option<u32> {
        Some(match self {
            Scorable::Dot => 1,
            Scorable::Energizer => 3,
            Scorable::Ghost => return None,
        })
    }
}
