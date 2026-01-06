use rand::{thread_rng, Rng};

pub fn private_key(p: u64) -> u64 {
    let mut rng = thread_rng();
    rng.gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modular_exponent(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modular_exponent(b_pub, a, p)
}

fn modular_exponent(base: u64, exponent: u64, modulus: u64) -> u64 {
    match modulus {
        1 => 0,
        _ => modular_exponent_helper(1, base % modulus, exponent, modulus),
    }
}

fn modular_exponent_helper(result: u64, base: u64, exponent: u64, modulus: u64) -> u64 {
    match exponent {
        0 => result,
        _ => modular_exponent_helper(
            match exponent % 2 {
                1 => (result * base) % modulus,
                _ => result,
            },
            (base * base) % modulus,
            exponent >> 1,
            modulus,
        ),
    }
}
