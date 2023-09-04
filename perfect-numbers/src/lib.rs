use std::cmp::Ordering::{Equal, Greater, Less};

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
    Some(
        match (1..=num / 2)
            .filter(|m| num % m == 0)
            .sum::<u64>()
            .cmp(&num)
        {
            Less => Classification::Deficient,
            Equal => Classification::Perfect,
            Greater => Classification::Abundant,
        },
    )
}
