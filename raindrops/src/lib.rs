type Drop = (u32, char);

pub fn raindrops(n: u32) -> String {
    let drops = vec![(3u32, 'i'), (5, 'a'), (7, 'o')]
        .iter()
        .map(|drop: &Drop| -> String {
            if n % drop.0 > 0 {
                "".to_owned()
            } else {
                format!("Pl{}ng", drop.1)
            }
        })
        .collect::<Vec<String>>()
        .join("");
    if drops.is_empty() {
        n.to_string()
    } else {
        drops
    }
}
