pub fn egg_count(display_value: u32) -> usize {
    if display_value == 0 {
        0
    } else if display_value % 2 == 1 {
        1 + egg_count(display_value - 1)
    } else {
        egg_count(display_value / 2)
    }
}
