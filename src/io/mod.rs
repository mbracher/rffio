use std::io;
use std::path::Path;

pub trait IoMethod {
    fn write(path: &Path, data: &[u8]) -> io::Result<()>;
    fn read(path: &Path) -> io::Result<usize>;
}

pub mod buffered;
pub mod mmap;

pub use buffered::{read_buffered, write_buffered, Buffered};
pub use mmap::{read_mmap, write_mmap, MmapIo};
