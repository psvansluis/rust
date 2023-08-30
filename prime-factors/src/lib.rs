pub fn factors(n: u64) -> Vec<u64> {
    let mut candidate_factor: u64 = 2;
    let mut current_n: u64 = n;
    let mut out: Vec<u64> = Vec::new();

    while current_n > 1 {
        if current_n % candidate_factor == 0 {
            out.push(candidate_factor);
            current_n /= candidate_factor;
        } else {
            candidate_factor += 1;
        }
    }
    out
}
