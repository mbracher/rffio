# Rust Fast File IO Experiments

This project benchmarks different approaches to reading and writing files in Rust. It
currently compares buffered I/O with memory–mapped files using [criterion].

## Running the benchmarks

```bash
cargo bench
```

The benchmarks will create temporary 10–MB files for each iteration and measure the
speed of reading and writing them with the available methods.

[criterion]: https://github.com/bheisler/criterion.rs
