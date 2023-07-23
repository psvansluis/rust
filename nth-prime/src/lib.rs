pub fn nth(n: u32) -> u32 {
    let mut primes = vec![2, 3];
    while primes.len() <= n as usize {
        grow_primes_list(&mut primes, 2);
    }
    primes[n as usize]
}

fn grow_primes_list(primes: &mut Vec<u32>, offset: u32) {
    let candidate = primes.last().unwrap() + offset;
    if primes.iter().any(|prime| candidate % prime == 0) {
        grow_primes_list(primes, offset + 2)
    } else {
        primes.push(candidate);
    }
}
