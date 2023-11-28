const VALID_LENGTHS: [usize; 2] = [10, 13];
const CHECK_CHARACTER: &char = &'X';
const DASH: &char = &'-';

/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let cleaned: Vec<char> = isbn.chars().filter(|ch| ch != DASH).collect();
    let len = cleaned.len();
    if !VALID_LENGTHS.contains(&len) {
        return false;
    }
    cleaned
        .iter()
        .enumerate()
        .try_fold(0, |acc, (i, ch)| {
            let multiplier: u32 = (len - i) as u32;
            if ch == CHECK_CHARACTER && multiplier == 1 {
                return Some(acc + 10);
            }
            ch.to_digit(10).map(|d| acc + (d * multiplier))
        })
        .is_some_and(|sum| sum % 11 == 0)
}
