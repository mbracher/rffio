#!/usr/bin/env bash
set -euo pipefail

SAMPLE_SIZE=${SAMPLE_SIZE:-10}
OUTPUT_DIR="benchmark_results"
mkdir -p "$OUTPUT_DIR"

# Run criterion benchmarks
cargo bench --bench io_bench -- --noplot --sample-size "$SAMPLE_SIZE" > "$OUTPUT_DIR/raw_output.txt"

BENCHES=(write_buffered write_mmap read_buffered read_mmap)
RESULTS_MD="$OUTPUT_DIR/results.md"
echo "| Benchmark | Time (ms) |" > "$RESULTS_MD"
echo "|-----------|---------:|" >> "$RESULTS_MD"

for b in "${BENCHES[@]}"; do
    JSON="target/criterion/$b/new/estimates.json"
    if [ -f "$JSON" ]; then
        ns=$(jq '.mean.point_estimate' "$JSON")
        ms=$(awk -v val="$ns" 'BEGIN{printf "%.3f", val/1e6}')
        echo "| $b | $ms |" >> "$RESULTS_MD"
    fi
done

echo "Results written to $RESULTS_MD"
