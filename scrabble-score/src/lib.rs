mod score_letter;

/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    word.chars().map(score_letter::score_letter).sum()
}
