use std::iter::once;

const LATIN_ALPHABET_LENGTH: i32 = 26;
const INDEX_A: i32 = 'a' as i32;
const CHUNK_SIZE: usize = 5;

/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    let encrypt = |ch| index_to_char(((a * char_to_index(ch)) + b) % LATIN_ALPHABET_LENGTH);
    match modular_multiplicative_inverse(a, LATIN_ALPHABET_LENGTH) {
        Some(_) => Ok(into_chunks(apply_code(plaintext, &encrypt))),
        _ => Err(AffineCipherError::NotCoprime(a)),
    }
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    let decrypt = |ch| {
        index_to_char(
            modular_multiplicative_inverse(a, LATIN_ALPHABET_LENGTH).unwrap()
                * (char_to_index(ch) - b),
        )
    };
    match modular_multiplicative_inverse(a, LATIN_ALPHABET_LENGTH) {
        Some(_) => Ok(String::from_iter(apply_code(ciphertext, &decrypt))),
        _ => Err(AffineCipherError::NotCoprime(a)),
    }
}

fn into_chunks(chs: Vec<char>) -> String {
    chs.into_iter()
        .enumerate()
        .flat_map(|(index, ch)| {
            match (index, index % CHUNK_SIZE) {
                (1.., 0) => Some(' '),
                _ => None,
            }
            .into_iter()
            .chain(once(ch))
        })
        .collect::<String>()
}

fn greatest_common_denominator(a: i32, b: i32) -> i32 {
    let mut pair: [i32; 2] = [a, b];
    pair.sort();
    match pair {
        [0, _] => 0,
        [a, b] if a == b => a,
        [a, b] => greatest_common_denominator(a, b - a),
    }
}

#[test]
fn gcd() {
    assert_eq!(4, greatest_common_denominator(12, 16));
    assert_eq!(1, greatest_common_denominator(4, 7));
}

fn modular_multiplicative_inverse(a: i32, m: i32) -> Option<i32> {
    match greatest_common_denominator(a, m) {
        1 => (1..).find(|n| (a * n) % m == 1),
        _ => None,
    }
}

#[test]
fn mmi() {
    assert_eq!(
        Some(3),
        modular_multiplicative_inverse(9, LATIN_ALPHABET_LENGTH)
    );
    assert_eq!(
        Some(7),
        modular_multiplicative_inverse(15, LATIN_ALPHABET_LENGTH)
    );
    assert_eq!(
        None,
        modular_multiplicative_inverse(18, LATIN_ALPHABET_LENGTH)
    );
}

fn apply_code(message: &str, f: &dyn Fn(char) -> char) -> Vec<char> {
    message
        .to_ascii_lowercase()
        .chars()
        .filter_map(|ch: char| match ch {
            'a'..='z' => Some(f(ch)),
            '0'..='9' => Some(ch),
            _ => None,
        })
        .collect::<Vec<char>>()
}

fn char_to_index(ch: char) -> i32 {
    ch as i32 - INDEX_A
}

#[test]
fn test_char_to_index() {
    assert_eq!(0, char_to_index('a'));
    assert_eq!(25, char_to_index('z'));
}

fn index_to_char(index: i32) -> char {
    match index {
        0.. => ((index % LATIN_ALPHABET_LENGTH + INDEX_A) as u8) as char,
        _ => index_to_char(index + LATIN_ALPHABET_LENGTH),
    }
}

#[test]
fn indextochar() {
    assert_eq!('a', index_to_char(0));
    assert_eq!('m', index_to_char(char_to_index('m')));
}
