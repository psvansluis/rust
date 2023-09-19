use std::fmt::Display;

fn primitive(n: u16) -> Option<char> {
    match n {
        1 => Some('I'),
        5 => Some('V'),
        10 => Some('X'),
        50 => Some('L'),
        100 => Some('C'),
        500 => Some('D'),
        1000 => Some('M'),
        _ => None,
    }
}
pub struct Roman(u16);

impl Display for Roman {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 == 0 {
            return write!(f, "");
        } else if let Some(p) = primitive(self.0) {
            return write!(f, "{p}");
        }
        let high: u16 = match self.0 {
            900.. => 1000,
            400.. => 500,
            90.. => 100,
            40.. => 50,
            9.. => 10,
            4.. => 5,
            0.. => 1,
        };

        let prefix: u16 = match self.0 {
            900..=1000 | 400..=500 => 100,
            90..=100 | 40..=50 => 10,
            9 | 4 => 1,
            _ => 0,
        };

        write!(
            f,
            "{}{}{}",
            Self(prefix),
            Self(high),
            Self(self.0 + prefix - high)
        )
    }
}

impl Roman {
    pub fn from(n: u16) -> Self {
        Self(n)
    }
}
