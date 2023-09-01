const ALPHABET_SIZE: u8 = 26;
use rand::{self, Rng};
use std::ops::{Add, Sub};

pub fn encode(key: &str, s: &str) -> Option<String> {
    Key::new(key).map(|k| k.apply(s, code_fun(&Add::add)))
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    Key::new(key).map(|k| k.apply(s, code_fun(&Sub::sub)))
}

pub fn encode_random(s: &str) -> (String, String) {
    let key = Key::random().0;
    let encoded = encode(&key, s).unwrap();
    (key, encoded)
}

fn code_fun<F>(operator: &'static F) -> impl Fn((char, char)) -> char
where
    F: Fn(u8, u8) -> u8,
{
    |(key_char, string_char): (char, char)| {
        let i = |char: char| -> u8 { char as u8 - b'a' };
        let lc = string_char.to_ascii_lowercase();
        let new_i = i(lc) + operator(ALPHABET_SIZE, i(key_char));
        (b'a' + (new_i % ALPHABET_SIZE)) as char
    }
}

struct Key(String);

impl Key {
    fn new(key: &str) -> Option<Self> {
        match key {
            "" => None,
            lc if lc.chars().all(|c| c.is_ascii_lowercase()) => Some(Key(lc.to_owned())),
            _ => None,
        }
    }

    fn apply<F>(&self, s: &str, coding_function: F) -> String
    where
        F: Fn((char, char)) -> char,
    {
        self.0
            .chars()
            .cycle()
            .zip(s.chars())
            .map(coding_function)
            .collect()
    }

    fn random() -> Self {
        let mut rng = rand::thread_rng();
        Key((0..100).map(|_| rng.gen_range('a'..='z')).collect())
    }
}
