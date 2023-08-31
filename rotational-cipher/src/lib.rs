pub fn rotate(input: &str, key: i8) -> String {
    match key {
        1.. => rotate(&input.chars().map(next).collect::<String>(), key - 1),
        0 => input.to_owned(),
        _ => rotate(input, key + 26),
    }
}

fn next(char: char) -> char {
    match char {
        'z' | 'Z' => (char as u8 - 25) as char,
        letter if char.is_ascii_alphabetic() => (letter as u8 + 1) as char,
        _ => char,
    }
}
