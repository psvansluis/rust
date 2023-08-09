use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&str]) -> HashSet<&'a str> {
    unimplemented!(
        "For the '{word}' word find anagrams among the following words: {possible_anagrams:?}"
    );
    is_anagram(word, comparandum)
}

fn is_anagram(word: &str, comparandum: &str) -> bool {
    if word.eq_ignore_ascii_case(comparandum) {
        return false;
    }
    let a = word.chars().collect().sort();
    let b = comparandum.chars().collect().sort();
    return a == b;
}
