pub fn build_proverb(items: &[&str]) -> String {
    let mut out: String = String::new();
    match items {
        [] => out,
        [head] => out + &build_last(head),
        [head, tail @ ..] => {
            for i in 0..tail.len() {
                out += &format!(
                    "For want of a {} the {} was lost.\n",
                    items[i],
                    items[i + 1]
                );
            }
            out + &build_last(head)
        }
    }
}

fn build_last(first_item: &str) -> String {
    format!("And all for the want of a {first_item}.")
}
