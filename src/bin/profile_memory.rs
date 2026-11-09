use dhat::{Alloc, Profiler};
use std::hint::black_box;

use rust_string_lookup_benchmark::finite_state_transducer::load_fst;
use rust_string_lookup_benchmark::hashmap::load_hashmap;
use rust_string_lookup_benchmark::trie::load_trie;

#[global_allocator]
static ALLOC: Alloc = Alloc;

fn main() {
    let _profiler = Profiler::new_heap();

    let tsv_path = "data/dataset_4m.tsv";
    let fst_path = "data/dataset_4m.fst";

    println!("Loading FST...");
    let fst_map = load_fst(fst_path).expect("Failed to load FST");
    black_box(fst_map);

    println!("Loading HashMap...");
    let hash_map = load_hashmap(tsv_path).expect("Failed to load HashMap");
    black_box(hash_map);

    println!("Loading Trie...");
    let trie_map = load_trie(tsv_path).expect("Failed to load Trie");
    black_box(trie_map);

    println!("Done. Writing DHAT profile...");
}
