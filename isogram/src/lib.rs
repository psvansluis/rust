pub fn check(candidate: &str) -> bool {
    let lc = candidate.to_ascii_lowercase();
    lc.chars()
        .filter(char::is_ascii_alphabetic)
        .all(|ch| lc.find(ch) == lc.rfind(ch))
}
