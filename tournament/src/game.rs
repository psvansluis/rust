use std::str::FromStr;

pub struct Game {
    pub home: String,
    pub away: String,
    pub outcome: Outcome,
}

impl FromStr for Game {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(";");
        let home = split
            .next()
            .ok_or(ParseError::InsufficientFields)?
            .to_owned();
        let away = split
            .next()
            .ok_or(ParseError::InsufficientFields)?
            .to_owned();
        let outcome = split
            .next()
            .ok_or(ParseError::InsufficientFields)?
            .parse()?;
        Ok(Game {
            home,
            away,
            outcome,
        })
    }
}

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

#[derive(Debug)]
pub enum ParseError {
    UnparseableOutcome,
    InsufficientFields,
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
