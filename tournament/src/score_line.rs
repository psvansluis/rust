use std::fmt;

use crate::outcome::Outcome;

pub struct ScoreLine {
    wins: u32,
    losses: u32,
    draws: u32,
}

impl ScoreLine {
    pub fn new_empty() -> ScoreLine {
        ScoreLine {
            wins: 0,
            losses: 0,
            draws: 0,
        }
    }

    pub fn add_outcome(&mut self, outcome: &Outcome) -> () {
        match outcome {
            Outcome::Win => self.wins += 1,
            Outcome::Draw => self.draws += 1,
            Outcome::Loss => self.losses += 1,
        }
    }

    fn matches_played(&self) -> u32 {
        self.wins + self.losses + self.draws
    }

    pub fn points(&self) -> u32 {
        (self.wins * 3) + self.draws
    }

    pub fn to_string(&self, name: &str) -> String {
        format!(
            "{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            name,
            self.matches_played(),
            self.wins,
            self.draws,
            self.losses,
            self.points()
        )
    }
}
