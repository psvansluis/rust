use std::collections::HashMap;

pub fn brackets_are_balanced(string: &str) -> bool {
    let brackets: HashMap<char, char> = HashMap::from([('{', '}'), ('(', ')'), ('[', ']')]);
    let mut unclosed_brackets: Vec<char> = Vec::new();
    for ch in string.chars() {
        if brackets.contains_key(&ch) {
            unclosed_brackets.push(ch);
        } else if brackets.values().any(|&b| b == ch) {
            match unclosed_brackets.pop() {
                Some(last) => {
                    if brackets.get(&last).unwrap_or(&' ') != &ch {
                        return false;
                    }
                }
                None => return false,
            }
        }
    }
    unclosed_brackets.is_empty()
}
