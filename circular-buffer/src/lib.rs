pub struct CircularBuffer<T: Clone> {
    index: usize,
    vec: Vec<Option<T>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T: Clone> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            index: 0,
            vec: vec![Option::None; capacity],
        }
    }

    fn next_index(&self) -> usize {
        (self.index + 1) % self.vec.len()
    }

    fn increment_index(&mut self) -> () {
        self.index = self.next_index();
    }

    fn is_empty(&self) -> bool {
        self.vec.iter().all(|el| el.is_none())
    }

    fn is_full(&self) -> bool {
        self.vec.iter().all(|el| el.is_some())
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.is_full() {
            Err(Error::FullBuffer)
        } else {
            Ok(self.write_non_full(element))
        }
    }

    fn write_non_full(&mut self, element: T) -> () {
        match self.vec[self.index] {
            None => {
                self.vec[self.index] = Some(element);
                self.increment_index();
            }
            Some(_) => {
                self.increment_index();
                self.write_non_full(element);
            }
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.is_empty() {
            Err(Error::EmptyBuffer)
        } else {
            Ok(self.read_non_empty())
        }
    }

    fn read_non_empty(&mut self) -> T {
        if let Some(el) = &self.vec[self.index] {
            let out = el.clone();
            self.vec[self.index] = None;
            self.increment_index();
            out
        } else {
            self.increment_index();
            self.read_non_empty()
        }
    }

    pub fn clear(&mut self) {
        *self = Self::new(self.vec.len());
    }

    pub fn overwrite(&mut self, element: T) {
        if !self.is_full() {
            self.write(element).unwrap();
        } else {
            self.vec[self.index] = Some(element);
            self.increment_index();
        }
    }
}
