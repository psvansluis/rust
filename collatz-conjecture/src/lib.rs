pub fn collatz(n: u64) -> Option<u64> {
    collatz_next(n, 0)
}

fn collatz_next(n: u64, steps_taken: u64) -> Option<u64> {
    match (n, n % 2) {
        (0, _) => None,
        (1, _) => Some(steps_taken),
        (_, 0) => collatz_next(n / 2, steps_taken + 1),
        (_, _) => n
            .checked_mul(3)
            .and_then(|multiplied| multiplied.checked_add(1))
            .and_then(|added| collatz_next(added, steps_taken + 1)),
    }
}
