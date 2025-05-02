use std::{any::Any, str::FromStr};

use crate::outcome::Outcome;

pub struct Game {
    home: String,
    away: String,
    outcome: Outcome,
}

impl FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(";");
        let home = split.next()?.to_owned();
        let away = split.next()?.to_owned();
        let outcome = split.next()?.parse()?;
        Ok(Game {
            home,
            away,
            outcome,
        })
    }
}
