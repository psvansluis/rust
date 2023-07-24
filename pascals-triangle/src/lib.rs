pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut triangle = Self { rows: Vec::new() };
        while triangle.rows.len() < row_count as usize {
            triangle.append_row();
        }
        triangle
    }

    fn append_row(&mut self) {
        match self.rows.last() {
            Some(_) => todo!(),
            None => self.rows.push(vec![1u32]),
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.to_owned()
    }
}
