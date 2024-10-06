pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    match (max_weight, items) {
        (0, _) | (_, []) => 0,
        (_, [Item { weight, value }, rest @ ..]) => max_weight
            .checked_sub(*weight)
            .map(|sub| value + maximum_value(sub, rest))
            .unwrap_or(0)
            .max(maximum_value(max_weight, rest)),
    }
}
