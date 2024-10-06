pub fn encode(n: u64) -> String {
    let encode_big = |floor: u64, suffix: &str| -> String {
        let remainder = if n % floor == 0 {
            "".to_owned()
        } else {
            " ".to_owned() + &encode(n % floor)
        };
        encode(n / floor) + " " + suffix + &remainder
    };
    match n {
        0 => "zero".to_owned(),
        1 => "one".to_owned(),
        2 => "two".to_owned(),
        3 => "three".to_owned(),
        4 => "four".to_owned(),
        5 => "five".to_owned(),
        6 => "six".to_owned(),
        7 => "seven".to_owned(),
        8 => "eight".to_owned(),
        9 => "nine".to_owned(),
        10 => "ten".to_owned(),
        11 => "eleven".to_owned(),
        12 => "twelve".to_owned(),
        13 => "thirteen".to_owned(),
        15 => "fifteen".to_owned(),
        18 => "eighteen".to_owned(),
        14..=19 => encode(n - 10) + "teen",
        20 => "twenty".to_owned(),
        30 => "thirty".to_owned(),
        40 => "forty".to_owned(),
        50 => "fifty".to_owned(),
        60 | 70 | 90 => encode(n / 10) + "ty",
        80 => "eighty".to_owned(),
        21..=99 => encode((n / 10) * 10) + "-" + &encode(n % 10),
        1_000_000_000_000_000_000.. => encode_big(1_000_000_000_000_000_000, "quintillion"),
        1_000_000_000_000_000.. => encode_big(1_000_000_000_000_000, "quadrillion"),
        1_000_000_000_000.. => encode_big(1_000_000_000_000, "trillion"),
        1_000_000_000.. => encode_big(1_000_000_000, "billion"),
        1_000_000.. => encode_big(1_000_000, "million"),
        1_000.. => encode_big(1_000, "thousand"),
        100.. => encode_big(100, "hundred"),
    }
}
