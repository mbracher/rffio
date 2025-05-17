# Rust Fast File IO Experiments

This project benchmarks different approaches to reading and writing files in Rust. It currently compares buffered I/O with memory–mapped files using [criterion].

## Running the benchmarks

```bash
cargo bench
```

The benchmarks will create temporary 10–MB files for each iteration and measure the speed of reading and writing them with the available methods.

## Collecting results

A helper script is provided to run the benchmarks and write a Markdown summary to `benchmark_results/results.md`:

```bash
./scripts/run_benchmarks.sh
```

The generated table can be included in this README to document the latest performance numbers.

[criterion]: https://github.com/bheisler/criterion.rs
