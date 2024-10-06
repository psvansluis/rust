pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

impl<T: ToString> Luhn for T {
    fn valid_luhn(&self) -> bool {
        self.to_string()
            .chars()
            .filter(|ch| !ch.is_whitespace())
            .map(|ch| ch.to_digit(10))
            .collect::<Option<Vec<u32>>>()
            .is_some_and(|digits| digits.len() > 1 && luhn_sum(&digits) % 10 == 0)
    }
}

fn luhn_sum(digits: &[u32]) -> u32 {
    let double_maybe = |digit: u32, tail_length: usize| -> u32 {
        match (digit, tail_length % 2) {
            (d, 0) => d,
            (d @ 5.., _) => (d * 2) - 9,
            (d, _) => d * 2,
        }
    };

    match digits {
        [] => 0,
        [head, tail @ ..] => double_maybe(*head, tail.len()) + luhn_sum(tail),
    }
}
