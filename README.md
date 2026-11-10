# rust-string-lookup-benchmark

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
string_lookup/fst        time:   [9.7335 ms 9.7592 ms 9.7953 ms]
                        thrpt:  [1.0209 Melem/s 1.0247 Melem/s 1.0274 Melem/s]
                 change:
                        time:   [−1.6411% −1.1055% −0.6025%] (p = 0.00 < 0.05)
                        thrpt:  [+0.6062% +1.1178% +1.6685%]
                        Change within noise threshold.
Found 9 outliers among 100 measurements (9.00%)
  6 (6.00%) high mild
  3 (3.00%) high severe
string_lookup/hashmap   time:   [697.88 µs 698.84 µs 700.25 µs]
                        thrpt:  [14.281 Melem/s 14.309 Melem/s 14.329 Melem/s]
                 change:
                        time:   [−38.002% −35.216% −32.375%] (p = 0.00 < 0.05)
                        thrpt:  [+47.874% +54.360% +61.295%]
                        Performance has improved.
Found 11 outliers among 100 measurements (11.00%)
  3 (3.00%) high mild
  8 (8.00%) high severe
string_lookup/trie      time:   [13.485 ms 13.492 ms 13.501 ms]
                        thrpt:  [740.66 Kelem/s 741.20 Kelem/s 741.58 Kelem/s]
                 change:
                        time:   [−8.7784% −7.8781% −7.0040%] (p = 0.00 < 0.05)
                        thrpt:  [+7.5315% +8.5519% +9.6232%]
                        Performance has improved.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe

```

## Memory Profiling

Memory profiling was conducted using [DHAT](https://valgrind.org/docs/manual/dh-manual.html) (`dhat-heap.json`) on `target/release/profile_memory` across the full dataset (4,665,810 entries).

### Summary

| Metric | HashMap (`AHashMap`) | Radix Trie (`fast_radix_trie`) | FST (`fst`) |
| --- | --- | --- | --- |
| **Total Allocated (`tb`)** | ~385.7 MB | ~615.1 MB | ~82.4 MB |
| **Peak Memory (`mb`)** | ~385.7 MB | ~111.3 MB | ~82.4 MB |
| **Allocation Count (`tbk`)** | 4,665,817 | 15,928,790 | 1 |
| **Retained at Exit (`gb`)** | **~385.7 MB** (100% live) | **0 B** (Freed) | **0 B** (Freed) |

### Key Findings

* **HashMap (`AHashMap`)**: Retains all data in memory at runtime (~385.7 MB). This consists of **276.8 MB** for the `hashbrown::raw::RawTable` bucket array in a single allocation, plus **108.9 MB** across 4.66M heap allocations for standard `String` keys.
* **Radix Trie (`fast_radix_trie`)**: Incurs heavy allocation churn during graph/node construction (~615.1 MB total allocated across ~15.9 million individual heap allocations). However, its peak memory remains modest (~111.3 MB), and all memory is completely deallocated when dropped (0 bytes retained at exit).
* **FST (`fst`)**: Most memory-efficient structure. Performs a single contiguous heap allocation of **82.4 MB** via `std::fs::read` to load the serialized state, which is fully freed after execution.
