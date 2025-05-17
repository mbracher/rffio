use std::fs::{File, OpenOptions};
use std::io::{self};
use std::path::Path;

use memmap2::{Mmap, MmapMut};

use super::IoMethod;

pub struct MmapIo;

impl IoMethod for MmapIo {
    fn write(path: &Path, data: &[u8]) -> io::Result<()> {
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

    fn read(path: &Path) -> io::Result<usize> {
        let file = File::open(path)?;
        let mmap = unsafe { Mmap::map(&file)? };
        Ok(mmap.len())
    }
}

pub fn write_mmap(path: &Path, data: &[u8]) -> io::Result<()> {
    MmapIo::write(path, data)
}

pub fn read_mmap(path: &Path) -> io::Result<usize> {
    MmapIo::read(path)
}
