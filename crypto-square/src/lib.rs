use itertools::Itertools;

pub fn encrypt(input: &str) -> String {
    let normalized = normalize(input);
    let row_size = n_columns(normalized.len());
    let get_column_at_index = |i| {
        normalized
            .chunks(row_size)
            .map(|chunk| chunk.get(i).unwrap_or(&' '))
            .collect::<String>()
    };
    (0..row_size).map(get_column_at_index).join(" ")
}

fn normalize(input: &str) -> Vec<char> {
    input
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .map(|ch: char| char::to_ascii_lowercase(&ch))
        .collect()
}

fn n_columns(input_length: usize) -> usize {
    (input_length as f32).sqrt().ceil() as usize
}
