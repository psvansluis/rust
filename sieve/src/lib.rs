pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    sieve((2..=upper_bound).collect())
}

fn sieve(xs: Vec<u64>) -> Vec<u64> {
    let Some(&first) = xs.first() else {
        return xs;
    };
    let rest = sieve(xs.into_iter().filter(|x| *x % first > 0).collect());
    [vec![first], rest].concat()
}
