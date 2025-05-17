pub mod io;

pub use io::{read_buffered, read_mmap, write_buffered, write_mmap};

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
