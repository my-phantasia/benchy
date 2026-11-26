# benchy

I made this for my Research Methods Semester 5 assignment, the code could use some optimization but i do think its enough.

## Dataset
- dataset from: [wikimedia](https://dumps.wikimedia.org/other/pageviews/2026/2026-02/pageviews-20260201-000000.gz)  
- dataset size after cleanup and sorted: 4665810 entries  
- original file size: 186.5 MiB  
- tsv (cleaned) file size: 117.5 MiB  
- fst file size: 78.6 MiB  

There are bash script included for downloading, extracting, cleaning and sorting.
```sh
./download_dataset.sh

```

## Benchmark

Benchmarks were conducted using Criterion on a laptop equipped with an Intel Core i5-10300H CPU.

To ensure consistent results:
- CPU cores were isolated.
- Turbo Boost was disabled to keep the CPU running within its baseline frequency.

```sh
string_lookup/fst      time:   [110.75 ms 111.86 ms 113.03 ms]
                       thrpt:  [884.76 Kelem/s 894.00 Kelem/s 902.91 Kelem/s]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

string_lookup/fst_mmap  time:   [117.21 ms 117.77 ms 118.46 ms]
                        thrpt:  [844.17 Kelem/s 849.09 Kelem/s 853.20 Kelem/s]
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe
  
string_lookup/hashmap   time:   [14.635 ms 14.734 ms 14.846 ms]
                        thrpt:  [6.7357 Melem/s 6.7869 Melem/s 6.8328 Melem/s]
Found 11 outliers among 100 measurements (11.00%)
  3 (3.00%) high mild
  8 (8.00%) high severe

string_lookup/trie      time:   [135.81 ms 136.38 ms 136.99 ms]
                        thrpt:  [729.98 Kelem/s 733.26 Kelem/s 736.31 Kelem/s]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe

```

## Memory Profiling

Memory profiling was conducted using [DHAT](https://valgrind.org/docs/manual/dh-manual.html) (`dhat-heap.json`) on `target/release/profile_memory` across the full dataset (4,665,810 entries).

### Summary

| Metric | HashMap (AHashMap) | Radix Trie (fast_radix_trie) | FST (Vec<u8>) | FST (Mmap) |
|---|---:|---:|---:|---:|
| Total Heap Allocated (MB) | ~385.7 MB | ~506.3 MB | ~82.4 MB | 0 B |
| Peak Heap Memory (MB) | ~385.7 MB | ~253.9 MB | ~82.4 MB | 0 B |
| Heap Allocation Count | 4,665,821 | 11,263,118 | 1 | 0 |
| Retained at Exit | 88 B (0 B map data) | 0 B (Freed) | 0 B (Freed) | 0 B |

### Key Findings

- **HashMap (`AHashMap`)**: Allocates and holds **~385.7 MB** while live—**276.8 MB** for the `hashbrown::raw::RawTable` bucket array in a single allocation, plus **108.9 MB** across 4.66M individual heap allocations for `String` keys. All map and key memory is freed on drop (`0 B` retained at exit, aside from `88 B` in `ahash`'s static `OnceBox` RNG seed cache).
- **Radix Trie (`fast_radix_trie`)**: Incurs heavy node-splitting churn during tree construction (**~506.3 MB** total across ~11.26 million individual node allocations), settling at **~253.9 MB** of live node memory at peak before deallocating completely on drop (`0 B` retained at exit).
- **FST (`Vec<u8>`)**: Loads the serialized automaton via `std::fs::read` in a single contiguous **82.4 MB** (`78.6 MiB`) heap allocation, which is freed when dropped.
- **FST (`Mmap`)**: Eliminates Rust heap allocations entirely (`0 B` in DHAT) by memory-mapping the `78.6 MiB` FST file directly via the OS virtual memory subsystem. Rather than copying the file into user-space heap memory, the kernel demand-pages blocks through the OS page cache while preserving nearly identical lookup throughput to the heap-backed FST.
