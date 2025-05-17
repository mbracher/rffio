use std::fs::File;
use std::io::{self, Read, Write, BufReader, BufWriter};
use std::path::Path;

use super::IoMethod;

pub struct Buffered;

impl IoMethod for Buffered {
    fn write(path: &Path, data: &[u8]) -> io::Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(data)?;
        writer.flush()
    }

    fn read(path: &Path) -> io::Result<usize> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf).map(|size| size)
    }
}

// Keep function style API for backward compatibility
pub fn write_buffered(path: &Path, data: &[u8]) -> io::Result<()> {
    Buffered::write(path, data)
}

pub fn read_buffered(path: &Path) -> io::Result<usize> {
    Buffered::read(path)
}
