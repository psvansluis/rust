use std::vec;

pub fn is_armstrong_number(num: u32) -> bool {
    let digits = num_to_digits(num);
    let n = digits.len() as u32;

    digits.into_iter().fold(0_u64, |acc, d| acc + d.pow(n)) == num as u64
}

fn num_to_digits(num: u32) -> Vec<u64> {
    if num > 9 {
        let mut v = num_to_digits(num / 10);
        v.push(num as u64 % 10);
        v
    } else {
        vec![num as u64]
    }
}
