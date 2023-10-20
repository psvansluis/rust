pub struct Luhn {
    code: String,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        if let Some(digits) = self.digits() {
            return digits.len() > 1 && Luhn::sum(&digits) % 10 == 0;
        }
        false
    }

    fn digits(&self) -> Option<Vec<u32>> {
        self.code
            .chars()
            .filter(|ch| !ch.is_whitespace())
            .map(|ch| ch.to_digit(10))
            .collect()
    }

    fn sum(digits: &[u32]) -> u32 {
        let double_maybe = |digit: u32, tail_length: usize| -> u32 {
            match (digit, tail_length % 2) {
                (d, 0) => d,
                (d @ 5.., _) => (d * 2) - 9,
                (d, _) => d * 2,
            }
        };

        match digits {
            [] => 0,
            [head, tail @ ..] => double_maybe(*head, tail.len()) + Luhn::sum(tail),
        }
    }
}

impl<T: ToString> From<T> for Luhn {
    fn from(input: T) -> Self {
        Luhn {
            code: input.to_string(),
        }
    }
}
