use std::cmp::Ordering;

use regex::Regex;

/// Type implementing arbitrary-precision decimal arithmetic
#[derive(Eq)]
pub struct Decimal {
    digits: Vec<u8>,
    digits_after_integer: usize,
    sign: Sign,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Sign {
    Plus = 1,
    Minus = 0,
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        // if !(Regex::new(r"^[-+]?\\d+\\.?\\d*$").unwrap().is_match(input)) {
        //     return None;
        // }
        let digits_after_integer: usize = match input.rfind('.') {
            Some(index) => input.len() - index - 1,
            None => 0,
        };

        let sign = if input.starts_with('-') {
            Sign::Minus
        } else {
            Sign::Plus
        };

        let mut digits: Vec<u8> = Vec::new();

        input.chars().for_each(|c: char| {
            if let Some(index) = c.to_digit(10) {
                digits.push(index as u8)
            }
        });

        return Some(Self {
            digits,
            digits_after_integer,
            sign,
        });
    }

    pub fn digits(&self) -> &[u8] {
        self.digits.as_ref()
    }

    pub fn digits_after_integer(&self) -> usize {
        self.digits_after_integer
    }

    pub fn sign(&self) -> &Sign {
        &self.sign
    }

    pub fn whole(&self) -> &[u8] {
        &self.digits[..(self.digits.len() - self.digits_after_integer())]
    }
}

impl PartialEq for Decimal {
    fn eq(&self, other: &Self) -> bool {
        self.digits == other.digits
            && self.digits_after_integer == other.digits_after_integer
            && self.sign == other.sign
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Decimal {
    fn cmp(&self, other: &Self) -> Ordering {
        if &self.sign > &other.sign {
            return Ordering::Greater;
        } else if &self.sign < &other.sign() {
            return Ordering::Less;
        } else {
            return Ordering::Equal;
        }
    }
}

pub fn main() {
    println!("hi");
    let d: Decimal = Decimal::try_from(&"-10.00001".to_string()).expect("should be valid");
    println!("{:?}", d.digits());
    println!("{:?}", d.sign());
    println!("{:?}", d.digits_after_integer());
    println!("{:?}", d.whole());
}
