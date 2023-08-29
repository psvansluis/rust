fn plant_for_char(ch: char) -> &'static str {
    match ch {
        'C' => "clover",
        'G' => "grass",
        'R' => "radishes",
        'V' => "violets",
        _ => panic!("unexpected plant!"),
    }
}

fn distance_from_a(ch: char) -> usize {
    ch as usize - 'A' as usize
}

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let index: usize = distance_from_a(student.chars().next().unwrap()) * 2;
    diagram
        .lines()
        .flat_map(|row| row[index..index + 2].chars().map(plant_for_char))
        .collect()
}
