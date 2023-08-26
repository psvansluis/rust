/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    Letters::new().into_iter().all(|l| l.is_found_in(sentence))
}

struct Letters {
    letter: char,
}

impl Letters {
    pub fn new() -> Self {
        Letters { letter: 'a' }
    }
}

pub trait CaseInsensitive {
    fn is_found_in(&self, sentence: &str) -> bool;
}

impl CaseInsensitive for char {
    fn is_found_in(&self, sentence: &str) -> bool {
        sentence.contains(self.to_ascii_lowercase()) || sentence.contains(self.to_ascii_uppercase())
    }
}

impl Iterator for Letters {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        self.letter = (self.letter as u8 + 1) as char;
        match self.letter {
            l if l.is_ascii_lowercase() => Some(l),
            _ => None,
        }
    }
}
