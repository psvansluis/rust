use std::str::FromStr;

use crate::{outcome::Outcome, parse_error::ParseError};

#[derive(Debug)]
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
