pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut triangle = Self { rows: Vec::new() };
        triangle.grow(row_count as usize);
        triangle
    }

    fn grow(&mut self, target_length: usize) {
        if self.rows.len() < target_length {
            self.append_row();
            self.grow(target_length);
        }
    }

    fn append_row(&mut self) {
        self.rows.push(match self.rows.last() {
            Some(last_row) => next_row(last_row),
            None => vec![1],
        });
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.to_owned()
    }
}

fn next_row(previous_row: &Vec<u32>) -> Vec<u32> {
    let mut out = [vec![0u32], previous_row.to_owned()].concat();
    let inverse = [previous_row.to_owned(), vec![0u32]].concat();

    for i in 0..out.len() {
        out[i] += inverse[i];
    }
    out
}
