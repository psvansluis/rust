pub fn build_proverb(items: &[&str]) -> String {
    let mut out: String = String::new();
    match items {
        [] => out,
        [head, tail @ ..] => {
            for i in 0..tail.len() {
                let [first, second] = items[i..i + 2] else {panic!()};
                out += &format!("For want of a {} the {} was lost.\n", first, second);
            }
            out + &format!("And all for the want of a {head}.")
        }
    }
}
