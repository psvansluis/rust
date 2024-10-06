pub struct Triangle {
    n_unique: usize,
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        let mut sortable = sides.to_vec();
        sortable.sort();
        match sortable[..] {
            [0, _, _] => None,
            [l, m, h] if l + m < h => None,
            _ => Some(Triangle {
                n_unique: sides
                    .into_iter()
                    .collect::<::std::collections::HashSet<_>>()
                    .len(),
            }),
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.n_unique == 1
    }

    pub fn is_scalene(&self) -> bool {
        self.n_unique == 3
    }

    pub fn is_isosceles(&self) -> bool {
        self.n_unique < 3
    }
}
