pub struct Game {
    home: &str,
    away: &str,
    outcome: Outcome,
}

impl Game {
    fn new(line: &str) -> Self {
        line.split(";")
    }
}
