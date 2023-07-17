use time::{ext::NumericalDuration, PrimitiveDateTime as DateTime};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let gigasecond = 1e9.seconds();
    return start + gigasecond;
}
