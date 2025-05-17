use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write, BufReader, BufWriter};
use std::path::Path;

use memmap2::{Mmap, MmapMut};

pub fn write_buffered(path: &Path, data: &[u8]) -> io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    writer.write_all(data)?;
    writer.flush()
}

pub fn read_buffered(path: &Path) -> io::Result<usize> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).map(|size| size)
}

pub fn write_mmap(path: &Path, data: &[u8]) -> io::Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;
    file.set_len(data.len() as u64)?;
    let mut mmap = unsafe { MmapMut::map_mut(&file)? };
    mmap[..].copy_from_slice(data);
    mmap.flush()
}

pub fn read_mmap(path: &Path) -> io::Result<usize> {
    let file = File::open(path)?;
    let mmap = unsafe { Mmap::map(&file)? };
    Ok(mmap.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_roundtrip_buffered() {
        let data = b"hello world";
        let file = NamedTempFile::new().unwrap();
        write_buffered(file.path(), data).unwrap();
        let size = read_buffered(file.path()).unwrap();
        assert_eq!(size, data.len());
    }

    #[test]
    fn test_roundtrip_mmap() {
        let data = b"hello mmap";
        let file = NamedTempFile::new().unwrap();
        write_mmap(file.path(), data).unwrap();
        let size = read_mmap(file.path()).unwrap();
        assert_eq!(size, data.len());
    }
}
