pub fn encode(source: &str) -> String {
    source
        .chars()
        .fold(Vec::new(), |mut acc: Vec<Run>, ch: char| -> Vec<Run> {
            match acc.last_mut() {
                Some(run) if run.ch == ch => {
                    *run = Run {
                        ch,
                        size: run.size + 1,
                    };
                }
                _ => {
                    acc.push(Run { ch, size: 1 });
                }
            }
            acc
        })
        .iter()
        .map(Run::encode)
        .collect()
}

struct Run {
    ch: char,
    size: usize,
}

impl Run {
    fn encode(&self) -> String {
        match self.size {
            0 => "".to_owned(),
            1 => self.ch.to_string(),
            _ => format!("{}{}", self.size, self.ch),
        }
    }

    fn decode(&self) -> String {
        self.ch.to_string().repeat(self.size)
    }
}

pub fn decode(source: &str) -> String {
    source
        .chars()
        .fold(
            (String::new(), String::new()),
            |acc: (String, String), ch: char| {
                if ch.is_numeric() {
                    return (acc.0 + &ch.to_string(), acc.1);
                } else {
                    let size: usize = acc.0.parse().unwrap_or(1);
                    return (String::new(), acc.1 + &Run { size, ch }.decode());
                }
            },
        )
        .1
}
