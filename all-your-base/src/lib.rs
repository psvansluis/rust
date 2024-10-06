#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(digits: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }
    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }
    let number = digits_to_number(digits, from_base)?;
    Ok(number_to_digits(number, to_base))
}

fn digits_to_number(digits: &[u32], from_base: u32) -> Result<u32, Error> {
    match digits {
        [] => Ok(0),
        [first, _rest @ ..] if first >= &from_base => Err(Error::InvalidDigit(*first)),
        [first, rest @ ..] => {
            let first_as_number = first * (from_base.pow(rest.len() as u32));
            let rest_as_number = digits_to_number(rest, from_base)?;
            Ok(first_as_number + rest_as_number)
        }
    }
}

fn number_to_digits(number: u32, to_base: u32) -> Vec<u32> {
    if number < to_base {
        return vec![number];
    }
    let init = number_to_digits(number / to_base, to_base);
    let last = vec![number % to_base];
    [init, last].concat()
}
