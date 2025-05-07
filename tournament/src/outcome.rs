use crate::parse_error::ParseError;
use std::str::FromStr;

#[derive(Debug)]
pub enum Outcome {
    Win,
    Draw,
    Loss,
}

impl Outcome {
    pub fn reverse(&self) -> Outcome {
        match self {
            Outcome::Win => Self::Loss,
            Outcome::Draw => Self::Draw,
            Outcome::Loss => Self::Win,
        }
    }
}

impl FromStr for Outcome {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "win" => Ok(Self::Win),
            "draw" => Ok(Self::Draw),
            "loss" => Ok(Self::Loss),
            _ => Err(ParseError::UnparseableOutcome),
        }
    }
}
