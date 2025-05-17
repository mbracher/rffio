use criterion::{criterion_group, criterion_main, Criterion};
use rffio::{read_buffered, read_mmap, write_buffered, write_mmap};
use tempfile::NamedTempFile;

const SIZE: usize = 10 * 1024 * 1024; // 10MB

fn bench_write_buffered(c: &mut Criterion) {
    let data = vec![0u8; SIZE];
    c.bench_function("write_buffered", |b| {
        b.iter(|| {
            let file = NamedTempFile::new().unwrap();
            write_buffered(file.path(), &data).unwrap();
        })
    });
}

fn bench_write_mmap(c: &mut Criterion) {
    let data = vec![0u8; SIZE];
    c.bench_function("write_mmap", |b| {
        b.iter(|| {
            let file = NamedTempFile::new().unwrap();
            write_mmap(file.path(), &data).unwrap();
        })
    });
}

fn bench_read_buffered(c: &mut Criterion) {
    let data = vec![1u8; SIZE];
    let file = NamedTempFile::new().unwrap();
    write_buffered(file.path(), &data).unwrap();
    let path = file.path().to_path_buf();
    c.bench_function("read_buffered", |b| {
        b.iter(|| {
            let size = read_buffered(&path).unwrap();
            assert_eq!(size, SIZE);
        })
    });
}

fn bench_read_mmap(c: &mut Criterion) {
    let data = vec![1u8; SIZE];
    let file = NamedTempFile::new().unwrap();
    write_buffered(file.path(), &data).unwrap();
    let path = file.path().to_path_buf();
    c.bench_function("read_mmap", |b| {
        b.iter(|| {
            let size = read_mmap(&path).unwrap();
            assert_eq!(size, SIZE);
        })
    });
}

criterion_group!(benches, bench_write_buffered, bench_write_mmap, bench_read_buffered, bench_read_mmap);
criterion_main!(benches);
