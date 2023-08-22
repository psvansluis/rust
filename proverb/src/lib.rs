pub fn build_proverb(items: &[&str]) -> String {
    let mut out: String = String::new();
    match items {
        [] => out,
        [head, tail @ ..] => {
            for i in 0..tail.len() {
                out += &format!(
                    "For want of a {} the {} was lost.\n",
                    items[i],
                    items[i + 1]
                );
            }
            out + &format!("And all for the want of a {head}.")
        }
    }
}
