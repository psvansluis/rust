use std::{ops::Add, vec};

/// Type implementing arbitrary-precision decimal arithmetic
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decimal {
    sign: Sign,
    digits: Vec<u8>,
    offset: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Sign {
    Positive,
    Negative,
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        Some(Decimal {
            sign: Self::get_sign(input)?,
            digits: Self::get_digits(input)?,
            offset: Self::get_offset(input)?,
        })
    }

    fn get_sign(input: &str) -> Option<Sign> {
        match input.chars().next() {
            Some('-') => Some(Sign::Negative),
            Some('+') => Some(Sign::Positive),
            Some(n) if n.is_ascii_digit() => Some(Sign::Positive),
            _ => None,
        }
    }

    fn get_digits(input: &str) -> Option<Vec<u8>> {
        input
            .replace(&['.', '+', '-'][..], "")
            .chars()
            .map(|ch| ch.to_digit(10).map(|d| d as u8))
            .collect()
    }

    fn get_offset(input: &str) -> Option<usize> {
        input.chars().rev().position(|c: char| c == '.')
    }
}

#[test]
fn new() {
    assert!(Decimal::try_from("aap").is_none());
    assert!(Decimal::try_from("100.1").is_some_and(|d| d.offset == 1
        && d.sign == Sign::Positive
        && d.digits == vec![1, 0, 0, 1]));
}

#[test]
fn getdigits() {
    assert_eq!(None, Decimal::get_digits("aap"));
    assert_eq!(Some(vec![0, 1, 2, 3]), Decimal::get_digits("+01.23"));
    assert_eq!(
        Some(vec![1, 2, 6, 7, 8, 9, 0, 0, 0, 1]),
        Decimal::get_digits("-126.7890001")
    );
}

#[test]
fn getsign() {
    assert_eq!(None, Decimal::get_sign(""));
    assert_eq!(None, Decimal::get_sign("aap"));
    assert_eq!(Some(Sign::Positive), Decimal::get_sign("+3.0"));
    assert_eq!(Some(Sign::Positive), Decimal::get_sign("4.0"));
    assert_eq!(Some(Sign::Negative), Decimal::get_sign("-9.3"));
}

#[test]
fn getoffset() {
    assert_eq!(Some(3), Decimal::get_offset("0.123"));
    assert_eq!(Some(1), Decimal::get_offset("512312.0"));
}
