use std::io::{Read, Result, Write};

pub struct ReadStats<R>{
    reader: R,
    reads: usize,
    bytes_through: usize,
}

impl<R: Read + std::fmt::Debug> ReadStats<R> {
    pub fn new(reader: R) -> ReadStats<R> {
        Self { reader, reads: 0, bytes_through: 0 }
    }

    pub fn get_ref(&self) -> &R {
        &self.reader
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read + std::fmt::Debug> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.reads += 1;
        self.reader.read(buf).map(|bytes| {
            self.bytes_through += bytes;
            bytes
        })
    }
}

pub struct WriteStats<W> {
    writer: W,
    writes: usize,
    bytes_through: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(writer: W) -> WriteStats<W> {
        Self { writer, writes: 0, bytes_through: 0 }
    }

    pub fn get_ref(&self) -> &W {
        &self.writer
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.writes += 1;
        self.writer.write(buf).map(|bytes| {
            self.bytes_through += bytes;
            bytes
        })
    }

    fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }
}
