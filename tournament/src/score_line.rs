use std::fmt;

use crate::outcome::Outcome;

pub struct ScoreLine {
    team: String,
    wins: u32,
    losses: u32,
    draws: u32,
}

impl ScoreLine {
    pub fn new_empty(team: String) -> Self {
        Self {
            team: team,
            wins: 0,
            losses: 0,
            draws: 0,
        }
    }

    pub fn add_outcome(self, outcome: Outcome) -> Self {
        match outcome {
            Outcome::Win => Self {
                wins: self.wins + 1,
                ..self
            },
            Outcome::Draw => Self {
                draws: self.draws + 1,
                ..self
            },
            Outcome::Loss => Self {
                losses: self.losses + 1,
                ..self
            },
        }
    }

    fn matches_played(&self) -> u32 {
        self.wins + self.losses + self.draws
    }

    fn points(&self) -> u32 {
        (self.wins * 3) + self.draws
    }
}

impl fmt::Display for ScoreLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            self.team,
            self.matches_played(),
            self.wins,
            self.draws,
            self.losses,
            self.points()
        )
    }
}

#[test]
fn format_drawn() {
    let line: ScoreLine = ScoreLine::new_empty("test team".to_string());
    let line_with_draw = line.add_outcome(Outcome::Draw);
    assert_eq!(
        "test team                      |  1 |  0 |  1 |  0 |  1",
        line_with_draw.to_string()
    );
}
