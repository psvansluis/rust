use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    to_map(note)
        .iter()
        .all(&|(word, count)| to_map(magazine).get(word).unwrap_or(&0) >= count)
}

fn to_map<'a>(words: &'a [&'a str]) -> HashMap<&'a &'a str, i32> {
    words.iter().fold(HashMap::new(), |mut map, word| {
        *map.entry(word).or_insert(0) += 1;
        map
    })
}
