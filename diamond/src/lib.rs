pub fn get_diamond(c: char) -> Vec<String> {
    let chars = ('A'..=c).collect::<Vec<char>>();
    match chars.len() {
        1 => vec!['A'.to_string()],
        len => chars
            .iter()
            .chain(chars.iter().rev().skip(1))
            .map(|ch| {
                let dist_from_a: usize = *ch as usize - 'A' as usize;
                let side = || " ".repeat(len - 1 - dist_from_a);
                let mid = || " ".repeat((dist_from_a * 2) - 1);
                match ch {
                    'A' => vec![side(), side()],
                    _ => vec![side(), mid(), side()],
                }
                .join(&ch.to_string())
            })
            .collect::<Vec<String>>(),
    }
}
