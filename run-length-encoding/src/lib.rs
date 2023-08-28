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
            1 => format!("{}", self.ch),
            _ => format!("{}{}", self.size, self.ch),
        }
    }
    fn decode(&self) -> String {
        format!("{}", self.ch).repeat(self.size)
    }
}

pub fn decode(source: &str) -> String {
    unimplemented!("Return the run-length decoding of {source}.");
}
