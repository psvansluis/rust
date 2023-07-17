use std::io::{Read, Result, Write};

pub type ReadStats<T> = Stats<T>;
pub type WriteStats<T> = Stats<T>;

pub struct Stats<T> {
    wrapped: T,
    bytes_through: usize,
    n_operations: usize,
}

impl<T> Stats<T> {
    pub fn new(wrapped: T) -> Stats<T> {
        Self {
            wrapped,
            bytes_through: 0,
            n_operations: 0,
        }
    }

    pub fn get_ref(&self) -> &T {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }
}

impl<R: Read> ReadStats<R> {
    pub fn reads(&self) -> usize {
        self.n_operations
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.n_operations += 1;
        let bytes = self.wrapped.read(buf)?;
        self.bytes_through += bytes;
        Ok(bytes)
    }
}

impl<W: Write> WriteStats<W> {
    pub fn writes(&self) -> usize {
        self.n_operations
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.n_operations += 1;
        let bytes = self.wrapped.write(buf)?;
        self.bytes_through += bytes;
        Ok(bytes)
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}
