use std::collections::HashMap;
pub fn word_count(words: &str) -> HashMap<String, u32> {
    words
        .split(is_not_letter_or_apostrophe)
        .filter(has_contents)
        .map(trim_apostrophes_and_make_lowercase)
        .fold(HashMap::new(), add_word_to_count)
}

fn is_not_letter_or_apostrophe(c: char) -> bool {
    !(c.is_alphanumeric() || c == '\'')
}

fn has_contents(w: &&str) -> bool {
    !w.is_empty()
}

fn trim_apostrophes_and_make_lowercase(w: &str) -> String {
    w.trim_matches('\'').to_lowercase()
}

fn add_word_to_count(mut m: HashMap<String, u32>, w: String) -> HashMap<String, u32> {
    *m.entry(w).or_insert(0) += 1;
    m
}
