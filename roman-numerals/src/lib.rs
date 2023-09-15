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

impl Roman {
    pub fn from(n: u16) -> Self {
        Self(n)
    }

    pub fn to_string(&self) -> String {
        if self.0 == 0 {
            return String::new();
        } else if let Some(p) = primitive(self.0) {
            return p.to_string();
        }
        let prim = |n| primitive(n).unwrap().to_string();
        let sub = |min: u16| Self::from(self.0 - min).to_string();

        match self.0 {
            1000.. => prim(1000) + &sub(1000),
            900.. => prim(100) + &prim(1000) + &sub(900),
            500.. => prim(500) + &sub(500),
            400.. => prim(100) + &prim(500) + &sub(400),
            100.. => prim(100) + &sub(100),
            90.. => prim(10) + &prim(100) + &sub(90),
            50.. => prim(50) + &sub(50),
            40.. => prim(10) + &prim(50) + &sub(40),
            10.. => prim(10) + &sub(10),
            9.. => prim(1) + &prim(10) + &sub(9),
            5.. => prim(5) + &sub(5),
            4.. => prim(1) + &prim(5) + &sub(4),
            1.. => prim(1) + &sub(1),
            _ => unreachable!(),
        }
    }
}
