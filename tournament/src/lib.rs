use std::collections::HashMap;

use game::Game;
use score_line::ScoreLine;

mod board;
mod game;
mod outcome;
mod parse_error;
mod score_line;
const HEADER: &'static str = "Team                           | MP |  W |  D |  L |  P";

pub fn tally(match_results: &str) -> String {
    let results = match_results
        .lines()
        .map(|line| line.parse::<Game>().unwrap());

    let mut score_lines: HashMap<String, ScoreLine> = HashMap::new();
    for game in results {
        score_lines
            .entry(game.home)
            .or_insert(ScoreLine::new_empty())
            .add_outcome(&game.outcome);
        score_lines
            .entry(game.away)
            .or_insert(ScoreLine::new_empty())
            .add_outcome(&game.outcome.reverse());
    }
    let mut list: Vec<(&String, &ScoreLine)> = score_lines.iter().collect();
    list.sort_by(|(team_a, score_a), (team_b, score_b)| {
        match (score_b.points().cmp(&score_a.points()), team_a.cmp(team_b)) {
            (std::cmp::Ordering::Equal, team_cmp) => team_cmp,
            (score_cmp, _) => score_cmp,
        }
    });
    let mut out = vec![HEADER.to_string()];
    for (name, scores) in &list {
        out.push(scores.to_string(name));
    }
    out.join("\n")
}
