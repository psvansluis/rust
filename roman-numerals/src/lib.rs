use std::fmt::Display;

pub struct Roman(u16);

impl Display for Roman {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 == 0 {
            return write!(f, "");
        } else if let Some(p) = match self.0 {
            1 => Some('I'),
            5 => Some('V'),
            10 => Some('X'),
            50 => Some('L'),
            100 => Some('C'),
            500 => Some('D'),
            1000 => Some('M'),
            _ => None,
        } {
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
            400..=499 | 900..=999 => 100,
            40..=49 | 90..=99 => 10,
            4 | 9 => 1,
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
