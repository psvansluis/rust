pub fn get_diamond(c: char) -> Vec<String> {
    let chars = ('A'..=c).collect::<Vec<char>>();
    match chars.len() {
        1 => vec!['A'.to_string()],
        len => chars
            .iter()
            .chain(chars.iter().rev().skip(1))
            .map(|ch| match ch {
                'A' => " ".repeat(len - 1) + &ch.to_string() + &" ".repeat(len - 1),
                c => {
                    let dist_from_a: usize = *c as usize - 'A' as usize;
                    " ".repeat(len - 1 - dist_from_a)
                        + &c.to_string()
                        + &" ".repeat((dist_from_a * 2) - 1)
                        + &c.to_string()
                        + &" ".repeat(len - 1 - dist_from_a)
                }
            })
            .collect::<Vec<String>>(),
    }
}
