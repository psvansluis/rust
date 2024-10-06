use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    possible_anagrams
        .iter()
        .filter(|anagram| is_anagram(word, anagram))
        .copied()
        .collect()
}

fn is_anagram(word: &str, comparandum: &str) -> bool {
    let [lc1, lc2] = [word, comparandum].map(&str::to_lowercase);
    if lc1 == lc2 {
        return false;
    }
    let [c1, c2] = [lc1, lc2].map(|str| {
        let mut out: Vec<char> = str.chars().collect();
        out.sort();
        out
    });
    c1 == c2
}
