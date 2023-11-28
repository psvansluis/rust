fn n_bottles(n: u32) -> String {
    (match n {
        0 => "No more bottles".to_owned(),
        1 => "1 bottle".to_owned(),
        _ => format!("{n} bottles"),
    } + " of beer")
}

fn result(n: u32) -> String {
    match n {
        0 => format!("Go to the store and buy some more, {}", { n_bottles(99) }),
        _ => format!(
            "Take {} down and pass it around, {}",
            if n == 1 { "it" } else { "one" },
            n_bottles(n - 1).to_ascii_lowercase()
        ),
    }
}

pub fn verse(n: u32) -> String {
    format!(
        "{} on the wall, {}.\n{} on the wall.\n",
        n_bottles(n),
        n_bottles(n).to_ascii_lowercase(),
        result(n)
    )
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .rev()
        .map(verse)
        .collect::<Vec<String>>()
        .join("\n")
}
