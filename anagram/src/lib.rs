use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&str]) -> HashSet<&'a str> {
    println!("{}", to_lowercase_sorted(word));
    unimplemented!(
        "For the '{word}' word find anagrams among the following words: {possible_anagrams:?}"
    );
}

fn to_lowercase_sorted(word: &str) -> &str {
    // let mut str = word.to_string();
    // str.make_ascii_lowercase();
    // str.chars().collect::<Vec<char>>().sort_unstable();
    word.to_string().make_ascii_uppercase();
    word
}
