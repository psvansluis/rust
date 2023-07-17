use std::fmt;

const DAY: i32 = 24 * HOUR;
const HOUR: i32 = 60;

#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
    minutes: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.minutes / HOUR, self.minutes % HOUR)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut mutable_minutes: i32 = (hours * HOUR) + minutes;

        while mutable_minutes < 0 {
            mutable_minutes += DAY
        }

        Self {
            minutes: mutable_minutes % DAY,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        return Clock::new(0, self.minutes + minutes);
    }
}
