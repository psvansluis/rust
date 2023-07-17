pub fn is_leap_year(year: u64) -> bool {
    let d = |n| year % n == 0;
    d(400) || !d(100) && d(4)
}
