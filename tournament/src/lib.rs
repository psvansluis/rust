use std::collections::HashMap;

use game::Game;
use score_line::ScoreLine;

mod game;
mod score_line;
const HEADER: &str = "Team                           | MP |  W |  D |  L |  P";

pub fn tally(match_results: &str) -> String {
    let score_lines = match_results
        .lines()
        .map(|line| line.parse::<Game>().unwrap())
        .fold(HashMap::new(), add_game);

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

fn add_game(
    mut score_lines: HashMap<String, ScoreLine>,
    Game {
        home,
        away,
        outcome,
    }: Game,
) -> HashMap<String, ScoreLine> {
    [(home, &outcome), (away, &outcome.reverse())]
        .into_iter()
        .for_each(|(team, outcome)| {
            score_lines
                .entry(team)
                .or_insert(ScoreLine::new_empty())
                .add_outcome(outcome);
        });
    score_lines
}
