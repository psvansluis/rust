pub fn square_of_sum(n: u32) -> u32 {
    u32::pow(get_sum_of_first(n), 2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    if n == 1 {
        return 1;
    } else {
        return u32::pow(n, 2) + sum_of_squares(n - 1);
    }
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}

fn get_sum_of_first(n: u32) -> u32 {
    (n * (n + 1)) / 2
}
