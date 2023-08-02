pub fn is_valid(code: &str) -> bool {
    match code
        .chars()
        .filter(|ch| !ch.is_whitespace())
        .map(|ch| ch.to_digit(10))
        .collect::<Option<Vec<u32>>>()
    {
        Some(digits) if digits.len() > 1 => luhn_sum(&digits) % 10 == 0,
        _ => false,
    }
}

fn luhn_sum(digits: &[u32]) -> u32 {
    match digits {
        [] => 0,
        [head, tail @ ..] => double_maybe(*head, tail.len()) + luhn_sum(tail),
    }
}

fn double_maybe(digit: u32, tail_length: usize) -> u32 {
    match (digit, tail_length % 2) {
        (d, 0) => d,
        (d @ 5.., _) => (d * 2) - 9,
        (d, _) => d * 2,
    }
}

#[test]
fn luhn_sum_works_valid() {
    let v = vec![4, 5, 3, 9, 3, 1, 9, 5, 0, 3, 4, 3, 6, 4, 6, 7];
    assert_eq!(luhn_sum(&v), 80);
}
#[test]
fn luhn_sum_works_invalid() {
    let v = vec![8, 2, 7, 3, 1, 2, 3, 2, 7, 3, 5, 2, 0, 5, 6, 9];
    assert_eq!(luhn_sum(&v), 57);
}
