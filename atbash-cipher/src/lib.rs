const A: u8 = 'a' as u8;
const Z: u8 = 'z' as u8;
const CHUNK_SIZE: usize = 5;

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
        .chars()
        .filter_map(|char| code_char(char))
        .enumerate()
        .flat_map(|(i, c)| match (i, i % CHUNK_SIZE) {
            (1.., 0) => vec![' ', c],
            _ => vec![c],
        })
        .collect()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher.chars().filter_map(|char| code_char(char)).collect()
}

fn code_char(char: char) -> Option<char> {
    match char {
        c if char.is_ascii_alphabetic() => Some((Z - c.to_ascii_lowercase() as u8 + A) as char),
        d if char.is_ascii_digit() => Some(d),
        _ => None,
    }
}
