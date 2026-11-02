# rust-string-lookup-benchmark

## Dataset
- dataset from: [wikimedia](https://dumps.wikimedia.org/other/pageviews/2026/2026-02/pageviews-20260201-000000.gz)  
- dataset size after cleanup and sorted: 4665810 entries  
- original file size: 186.5 MiB  
- tsv (cleaned) file size: 117.5 MiB  
- fst file size: 78.6 MiB

There are bash script included for downloading, extraction, cleaning and sorting.
```sh
./download_dataset.sh
```
