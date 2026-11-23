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
string_lookup/fst       time:   [11.128 ms 11.255 ms 11.391 ms]
                        thrpt:  [877.91 Kelem/s 888.48 Kelem/s 898.65 Kelem/s]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

string_lookup/fst_mmap  time:   [11.547 ms 11.601 ms 11.669 ms]
                        thrpt:  [856.95 Kelem/s 862.01 Kelem/s 866.05 Kelem/s]
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe
  
string_lookup/hashmap   time:   [697.88 µs 698.84 µs 700.25 µs]
                        thrpt:  [14.281 Melem/s 14.309 Melem/s 14.329 Melem/s]
Found 11 outliers among 100 measurements (11.00%)
  3 (3.00%) high mild
  8 (8.00%) high severe

string_lookup/trie      time:   [13.485 ms 13.492 ms 13.501 ms]
                        thrpt:  [740.66 Kelem/s 741.20 Kelem/s 741.58 Kelem/s]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe

```

## Memory Profiling

Memory profiling was conducted using [DHAT](https://valgrind.org/docs/manual/dh-manual.html) (`dhat-heap.json`) on `target/release/profile_memory` across the full dataset (4,665,810 entries).

### Summary

| Metric | HashMap (AHashMap) | Radix Trie (fast_radix_trie) | FST (Vec<u8>) | FST (Mmap) |
|---|---:|---:|---:|---:|
| Total Heap Allocated (MB) | ~385.7 MB | ~615.1 MB | ~82.4 MB | 0 B |
| Peak Heap Memory (MB) | ~385.7 MB | ~111.3 MB | ~82.4 MB | 0 B |
| Heap Allocation Count | 4,665,817 | 15,928,790 | 1 | 0 |
| Retained at Exit | ~385.7 MB (100% live) | 0 B (Freed) | 0 B (Freed) | 0 B |

### Key Findings

* **HashMap (`AHashMap`)**: Retains all data in memory at runtime (~385.7 MB). This consists of **276.8 MB** for the `hashbrown::raw::RawTable` bucket array in a single allocation, plus **108.9 MB** across 4.66M heap allocations for standard `String` keys.
* **Radix Trie (`fast_radix_trie`)**: Incurs heavy allocation churn during graph/node construction (~615.1 MB total allocated across ~15.9 million individual heap allocations). However, its peak memory remains modest (~111.3 MB), and all memory is completely deallocated when dropped (0 bytes retained at exit).
* **FST (`Vec<u8>`)**: Loads the serialized state via `std::fs::read` in a single contiguous 82.4 MB (78.6 MiB) heap allocation, which is freed when dropped
* **FST (`Mmap`)**: Eliminates Rust heap allocations entirely (0 B in DHAT) by memory-mapping the 78.6 MiB FST file directly via the OS virtual memory subsystem. Rather than copying the file into user-space heap memory, the kernel demand-pages blocks through the OS page cache while preserving the same lookup throughput as the heap-backed FST.
