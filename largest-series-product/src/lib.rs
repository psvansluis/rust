#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    match span {
        0 => Ok(1),
        _ if span > string_digits.len() => Err(Error::SpanTooLong),
        _ => {
            let chars: Vec<u64> = string_digits
                .chars()
                .map(|char| {
                    char.to_digit(10)
                        .map(u64::from)
                        .ok_or(Error::InvalidDigit(char))
                })
                .collect::<Result<Vec<u64>, Error>>()?;
            let product = |start_index: usize| -> u64 {
                chars[start_index..start_index + span]
                    .iter()
                    .fold(1, |acc, &el| acc * el)
            };
            let mut max: u64 = 0;
            for i in 0..=chars.len() - span {
                let prod: u64 = product(i);
                if prod > max {
                    max = prod;
                }
            }
            return Ok(max);
        }
    }
}
