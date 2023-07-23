pub fn nth(n: u32) -> u32 {
    let mut primes = vec![2, 3];
    grow_primes_until(&mut primes, n as usize);
    primes[n as usize]
}

fn grow_primes_until(primes: &mut Vec<u32>, target_length: usize) {
    if primes.len() <= target_length {
        grow_primes_list(primes, 2);
        grow_primes_until(primes, target_length);
    }
}

fn grow_primes_list(primes: &mut Vec<u32>, offset: u32) {
    let candidate: u32 = primes.last().unwrap() + offset;
    if primes.iter().any(|prime| candidate % prime == 0) {
        grow_primes_list(primes, offset + 2);
    } else {
        primes.push(candidate);
    }
}
