use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use fst::{Automaton, IntoStreamer, Streamer, automaton::Str};
use rand::rng;
use rand::seq::SliceRandom;
use std::fs::File;
use std::hint::black_box;
use std::io::{BufRead, BufReader};

use rust_string_lookup_benchmark::finite_state_transducer::{bake_fst, load_fst, load_fst_mmap};
use rust_string_lookup_benchmark::hashmap::load_hashmap;
use rust_string_lookup_benchmark::trie::load_trie;

// Helper function to load a random subset of keys
fn load_sample_keys(tsv_path: &str, sample_size: usize) -> Vec<String> {
    let file = File::open(tsv_path).expect("Failed to open TSV");
    let reader = BufReader::new(file);
    let mut keys = Vec::new();

    for line in reader.lines().flatten() {
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            if let Some((key, _)) = trimmed.split_once('\t') {
                keys.push(key.to_string());
            }
        }
    }

    let mut rng = rng();
    keys.shuffle(&mut rng);
    keys.truncate(sample_size);
    keys
}

fn bench_lookup(c: &mut Criterion) {
    let tsv_path = "data/dataset_4m.tsv";
    let fst_path = "data/dataset_4m.fst";

    bake_fst(tsv_path, fst_path).expect("Failed to bake FST");
    let fst_map = load_fst(fst_path).expect("Failed to load FST");
    let fst_memmap = load_fst_mmap(fst_path).expect("Failed to memory mapped FST");
    let hash_map = load_hashmap(tsv_path).expect("Failed to load HashMap");
    let trie_map = load_trie(tsv_path).expect("Failed to load Trie");

    let sample_size = 100_000;
    let search_keys = load_sample_keys(tsv_path, sample_size);

    let mut lookup_group = c.benchmark_group("string_lookup");

    lookup_group.throughput(Throughput::Elements(search_keys.len() as u64));

    lookup_group.bench_function("fst", |b| {
        b.iter(|| {
            for key in &search_keys {
                let result = fst_map.get(black_box(key));
                black_box(result);
            }
        })
    });

    lookup_group.bench_function("fst_mmap", |b| {
        b.iter(|| {
            for key in &search_keys {
                let result = fst_memmap.get(black_box(key));
                black_box(result);
            }
        });
    });

    lookup_group.bench_function("hashmap", |b| {
        b.iter(|| {
            for key in &search_keys {
                let result = hash_map.get(black_box(key));
                black_box(result);
            }
        });
    });

    lookup_group.bench_function("trie", |b| {
        b.iter(|| {
            for key in &search_keys {
                let result = trie_map.get(black_box(key));
                black_box(result);
            }
        });
    });

    lookup_group.finish();

    const MAX_RESULTS: usize = 50;

    let search_prefixes: Vec<&str> = search_keys
        .iter()
        .take(1_000)
        .map(|k| match k.char_indices().nth(4) {
            Some((idx, _)) => &k[..idx],
            None => k.as_str(),
        })
        .collect();

    let mut prefix_group = c.benchmark_group("string_prefix_lookup");
    prefix_group.throughput(Throughput::Elements(search_prefixes.len() as u64));

    prefix_group.bench_function("fst_starts_with_top50", |b| {
        b.iter(|| {
            for &prefix in &search_prefixes {
                let matcher = Str::new(black_box(prefix)).starts_with();
                let mut stream = fst_map.search(matcher).into_stream();
                let mut count = 0;
                while let Some((k, v)) = stream.next() {
                    black_box((k, v));
                    count += 1;
                    if count >= MAX_RESULTS {
                        break;
                    }
                }
            }
        });
    });

    prefix_group.bench_function("fst_mmap_starts_with_top50", |b| {
        b.iter(|| {
            for &prefix in &search_prefixes {
                let matcher = Str::new(black_box(prefix)).starts_with();
                let mut stream = fst_memmap.search(matcher).into_stream();
                let mut count = 0;
                while let Some((k, v)) = stream.next() {
                    black_box((k, v));
                    count += 1;
                    if count >= MAX_RESULTS {
                        break;
                    }
                }
            }
        });
    });

    prefix_group.bench_function("trie_iter_prefix_top50", |b| {
        b.iter(|| {
            for &prefix in &search_prefixes {
                for entry in trie_map.iter_prefix(black_box(prefix)).take(MAX_RESULTS) {
                    black_box(entry);
                }
            }
        });
    });

    prefix_group.finish();
}

criterion_group!(benches, bench_lookup);
criterion_main!(benches);
