#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    match (1..=num / 2).filter(|m| num % m == 0).sum::<u64>() {
        _ if num == 0 => None,
        equal if equal == num => Some(Classification::Perfect),
        less if less < num => Some(Classification::Deficient),
        _more => Some(Classification::Abundant),
    }
}
