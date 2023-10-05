use std::cmp::Ordering::{self, Equal, Greater, Less};
use Classification::{Abundant, Deficient, Perfect};

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }
    Some(match order(num) {
        Less => Deficient,
        Equal => Perfect,
        Greater => Abundant,
    })
}

fn order(n: u64) -> Ordering {
    (1..=n / 2).filter(|m| n % m == 0).sum::<u64>().cmp(&n)
}
