use criterion::{Criterion, criterion_group, criterion_main};
use rust_string_lookup_benchmark::finite_state_transducer::{bake_fst, load_fst};
use rust_string_lookup_benchmark::hashmap::load_hashmap;
use rust_string_lookup_benchmark::trie::load_trie;
use std::hint::black_box;

fn bench_fst_lookup(c: &mut Criterion) {
    // Prepare data
    let tsv_path = "data/dataset_4m.tsv";
    let fst_path = "data/dataset_4m.fst";

    // Setup FST
    bake_fst(tsv_path, fst_path).expect("Failed to bake FST");
    let fst_map = load_fst(fst_path).expect("Failed to load FST");

    // Setup HashMap
    let hash_map = load_hashmap(tsv_path).expect("Failed to load HashMap");

    // Setup Trie
    let trie_map = load_trie(tsv_path).expect("Failed to load Trie");

    let search_key = "!Bang!";

    let mut group = c.benchmark_group("string_lookup");

    group.bench_function("fst", |b| {
        b.iter(|| {
            let result = fst_map.get(black_box(search_key));
            black_box(result);
        })
    });

    group.bench_function("hashmap", |b| {
        b.iter(|| {
            let result = hash_map.get(black_box(search_key));
            black_box(result);
        });
    });

    group.bench_function("trie", |b| {
        b.iter(|| {
            let result = trie_map.get(black_box(search_key));
            black_box(result);
        });
    });
}

criterion_group!(benches, bench_fst_lookup);
criterion_main!(benches);
